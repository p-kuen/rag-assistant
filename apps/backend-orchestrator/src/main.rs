use axum::{
    Json, Router,
    extract::{Multipart, State},
    http::StatusCode,
    response::{Sse, sse::Event},
    routing::{get, post},
};
use std::{convert::Infallible, sync::Arc, time::Duration};
use tower_http::cors::CorsLayer;
use tracing::info;

mod error;
mod ingestion;
mod models;
mod rag;
mod services;

use models::{DocumentListResponse, UploadResponse};
use services::MeilisearchService;

use crate::models::{ChatRequest, InputDocument};

// Application state
#[derive(Clone)]
struct AppState {
    meilisearch_service: Arc<MeilisearchService>,
    // generation_service: Arc<GenerationService>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize services
    let meilisearch_service = Arc::new(
        MeilisearchService::new()
            .await
            .map_err(|err| err.to_string())?,
    );
    // let generation_service = Arc::new(GenerationService::new()?);

    let app_state = AppState {
        meilisearch_service,
        // generation_service,
    };

    // Create router with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(chat_handler))
        .route("/api/documents", post(upload_document_handler))
        .route("/api/documents", get(list_documents_handler))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let addr = listener.local_addr().unwrap();
    info!("Backend-Orchestrator läuft auf {}", addr);

    axum::serve(listener, app.into_make_service())
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

    let user_message = payload.message.clone();

    let stream = async_stream::stream! {
        let res = state.meilisearch_service.hybrid_search(&user_message, 5, None).await;

        match res {
            Ok(results) => {
                for result in results {
                    let json = serde_json::to_string(&result).unwrap_or_default();
                    let event = Event::default().data(json);
                    yield Ok(event);
                }
            },
            Err(err) => {
                let error_event = Event::default().data(format!("Search error: {}", err));
                yield Ok(error_event);
            },
        }

        let done_event = Event::default().data("[DONE]");
        yield Ok(done_event);
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
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    let mut title = None;
    let mut content = String::new();
    let mut metadata = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name() {
            Some("file") => {
                if let Some(filename) = field.file_name() {
                    title = Some(filename.to_string());
                }
                if let Ok(file_content) = field.bytes().await {
                    content = String::from_utf8_lossy(&file_content).to_string();
                }
            }
            Some("title") => {
                if let Ok(title_bytes) = field.bytes().await {
                    title = Some(String::from_utf8_lossy(&title_bytes).to_string());
                }
            }
            Some("content") => {
                if let Ok(content_bytes) = field.bytes().await {
                    content = String::from_utf8_lossy(&content_bytes).to_string();
                }
            }
            Some("metadata") => {
                if let Ok(metadata_bytes) = field.bytes().await {
                    if let Ok(parsed_metadata) =
                        serde_json::from_slice::<models::DocumentMetadata>(&metadata_bytes)
                    {
                        metadata = Some(parsed_metadata);
                    }
                }
            }
            _ => {}
        }
    }

    if content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Content is empty".to_owned()));
    }

    let id = uuid::Uuid::now_v7().to_string();

    state
        .meilisearch_service
        .index_documents(vec![InputDocument {
            id,
            title,
            content,
            metadata,
            source_file: None,
        }])
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Json(UploadResponse {
        status: "processing".to_string(),
    }))
}

// Dokumente auflisten
async fn list_documents_handler(
    State(state): State<AppState>,
) -> Result<Json<DocumentListResponse>, StatusCode> {
    match state.meilisearch_service.list_documents().await {
        Ok(documents) => Ok(Json(DocumentListResponse { documents })),
        Err(e) => {
            tracing::error!("Failed to list documents: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
