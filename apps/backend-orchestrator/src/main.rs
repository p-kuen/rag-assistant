use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{sse::Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio_stream::StreamExt as TokioStreamExt;
use tower_http::cors::CorsLayer;
use tracing::info;

mod error;
mod ingestion;
mod models;
mod rag;
mod services;

use error::Result;
use ingestion::{IngestionPipeline, MarkdownParser};
use models::{
    create_task_store, generate_task_id, ChatRequest, ChatResponse, DocumentInfo,
    DocumentListResponse, TaskStatus, TaskStore, UploadRequest, UploadResponse,
};
use rag::{GenerationService, RetrievalService};

// Application state
#[derive(Clone)]
struct AppState {
    task_store: TaskStore,
    ingestion_pipeline: Arc<IngestionPipeline>,
    retrieval_service: Arc<RetrievalService>,
    generation_service: Arc<GenerationService>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize services
    let task_store = create_task_store();
    let ingestion_pipeline = Arc::new(IngestionPipeline::new(task_store.clone()).await?);
    let retrieval_service = Arc::new(RetrievalService::new()?);
    let generation_service = Arc::new(GenerationService::new()?);

    let app_state = AppState {
        task_store,
        ingestion_pipeline,
        retrieval_service,
        generation_service,
    };

    // Create router with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(chat_handler))
        .route("/api/documents", post(upload_document_handler))
        .route("/api/documents", get(list_documents_handler))
        .route("/api/documents/tasks/:task_id", get(get_task_status_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Backend-Orchestrator läuft auf {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Health Check Endpoint
async fn health_check() -> &'static str {
    "OK"
}

// Chat Endpoint - Orchestriert RAG Pipeline mit Streaming
async fn chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    info!("Chat Request empfangen: {}", payload.message);

    let retrieval_service = state.retrieval_service.clone();
    let generation_service = state.generation_service.clone();
    let user_message = payload.message.clone();

    let stream = async_stream::stream! {
        // Step 1: Retrieve relevant chunks
        match retrieval_service.retrieve_with_context(&user_message, 5).await {
            Ok((sources, context)) => {
                // Send sources as initial event
                let sources_event = Event::default()
                    .json_data(sources.clone())
                    .unwrap_or_else(|_| Event::default().data("sources"));
                yield Ok(sources_event);

                // Step 2: Generate streaming response
                match generation_service.generate_response_stream(&user_message, &context, sources).await {
                    Ok(mut token_stream) => {
                        while let Some(token_result) = token_stream.next().await {
                            match token_result {
                                Ok(token) => {
                                    let event = Event::default().data(token);
                                    yield Ok(event);
                                }
                                Err(e) => {
                                    let error_event = Event::default().data(format!("Error: {}", e));
                                    yield Ok(error_event);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let error_event = Event::default().data(format!("Generation error: {}", e));
                        yield Ok(error_event);
                    }
                }
            }
            Err(e) => {
                let error_event = Event::default().data(format!("Retrieval error: {}", e));
                yield Ok(error_event);
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive-text"),
    )
}

// Dokument Upload Endpoint (Multipart)
async fn upload_document_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, StatusCode> {
    let mut title = String::new();
    let mut content = String::new();
    let mut metadata = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("file") => {
                if let Some(filename) = field.file_name() {
                    title = filename.to_string();
                }
                if let Ok(file_content) = field.bytes().await {
                    content = String::from_utf8_lossy(&file_content).to_string();
                }
            }
            Some("title") => {
                if let Ok(title_bytes) = field.bytes().await {
                    title = String::from_utf8_lossy(&title_bytes).to_string();
                }
            }
            Some("content") => {
                if let Ok(content_bytes) = field.bytes().await {
                    content = String::from_utf8_lossy(&content_bytes).to_string();
                }
            }
            Some("metadata") => {
                if let Ok(metadata_bytes) = field.bytes().await {
                    if let Ok(parsed_metadata) = serde_json::from_slice::<models::DocumentMetadata>(&metadata_bytes) {
                        metadata = Some(parsed_metadata);
                    }
                }
            }
            _ => {}
        }
    }

    if content.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let task_id = generate_task_id();
    state.ingestion_pipeline.create_task(task_id.clone()).await;

    // Process document asynchronously
    let pipeline = state.ingestion_pipeline.clone();
    let task_id_clone = task_id.clone();

    tokio::spawn(async move {
        if title.ends_with(".md") || content.contains("#") {
            // Process as Markdown document
            match MarkdownParser::parse_document(&content, &title) {
                Ok(document) => {
                    if let Err(e) = pipeline.process_document(document, task_id_clone).await {
                        tracing::error!("Failed to process document: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to parse document: {}", e);
                }
            }
        } else {
            // Process as plain text
            if let Err(e) = pipeline.process_text(content, title, task_id_clone).await {
                tracing::error!("Failed to process text: {}", e);
            }
        }
    });

    Ok(Json(UploadResponse {
        task_id,
        status: "processing".to_string(),
    }))
}

// Task Status Endpoint
async fn get_task_status_handler(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskStatus>, StatusCode> {
    match state.ingestion_pipeline.get_task_status(&task_id).await {
        Some(status) => Ok(Json(status)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Dokumente auflisten
async fn list_documents_handler(
    State(state): State<AppState>,
) -> Result<Json<DocumentListResponse>, StatusCode> {
    match state.retrieval_service.meilisearch_service.list_documents().await {
        Ok(documents) => Ok(Json(DocumentListResponse { documents })),
        Err(e) => {
            tracing::error!("Failed to list documents: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
