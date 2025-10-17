use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // Logging initialisieren
    tracing_subscriber::fmt::init();

    // Router mit Routen erstellen
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(chat_handler))
        .route("/api/documents", post(upload_document_handler))
        .route("/api/documents", get(list_documents_handler))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Backend-Orchestrator läuft auf {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Health Check Endpoint
async fn health_check() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    session_id: Option<String>,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
    sources: Vec<Source>,
}

#[derive(Serialize)]
struct Source {
    document_id: String,
    chunk_id: String,
    score: f32,
}

// Chat Endpoint - Orchestriert RAG Pipeline
async fn chat_handler(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    info!("Chat Request empfangen: {}", payload.message);

    // TODO: Implementierung der RAG-Pipeline:
    // 1. Embedding-Service aufrufen für Benutzer-Query
    // 2. Meilisearch abfragen für relevante Chunks
    // 3. LLM-Service mit Context aufrufen
    // 4. Response zurückgeben

    Json(ChatResponse {
        response: "Placeholder Response".to_string(),
        sources: vec![],
    })
}

#[derive(Deserialize)]
struct UploadRequest {
    title: String,
    content: String,
}

#[derive(Serialize)]
struct UploadResponse {
    task_id: String,
    status: String,
}

// Dokument Upload Endpoint
async fn upload_document_handler(Json(payload): Json<UploadRequest>) -> Json<UploadResponse> {
    info!("Dokument Upload: {}", payload.title);

    // TODO: Implementierung:
    // 1. Dokument chunking
    // 2. Embeddings generieren
    // 3. In Meilisearch speichern

    Json(UploadResponse {
        task_id: "task-123".to_string(),
        status: "pending".to_string(),
    })
}

#[derive(Serialize)]
struct DocumentListResponse {
    documents: Vec<DocumentInfo>,
}

#[derive(Serialize)]
struct DocumentInfo {
    id: String,
    title: String,
    status: String,
    created_at: String,
}

// Dokumente auflisten
async fn list_documents_handler() -> Json<DocumentListResponse> {
    // TODO: Aus Meilisearch abrufen

    Json(DocumentListResponse {
        documents: vec![],
    })
}
