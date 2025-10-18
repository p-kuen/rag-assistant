use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub document_type: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub id: String,
    pub content: String,
    pub metadata: DocumentMetadata,
    pub hierarchy_lvl1: Option<String>,
    pub hierarchy_lvl2: Option<String>,
    pub hierarchy_lvl3: Option<String>,
    pub chunk_index: usize,
    pub source_file: String,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub content: String,
    pub metadata: DocumentMetadata,
    pub chunks: Vec<DocumentChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub metadata: DocumentMetadata,
    pub hierarchy_lvl1: Option<String>,
    pub hierarchy_lvl2: Option<String>,
    pub hierarchy_lvl3: Option<String>,
    pub chunk_index: usize,
    pub source_file: String,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub response: String,
    pub sources: Vec<SearchResult>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequest {
    pub title: String,
    pub content: String,
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub task_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub id: String,
    pub status: TaskStatusType,
    pub progress: Option<f32>,
    pub error: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatusType {
    Pending,
    Processing,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub id: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub chunk_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentListResponse {
    pub documents: Vec<DocumentInfo>,
}

// Task management
pub type TaskStore = std::sync::Arc<std::sync::RwLock<HashMap<String, TaskStatus>>>;

pub fn create_task_store() -> TaskStore {
    std::sync::Arc::new(std::sync::RwLock::new(HashMap::new()))
}

pub fn generate_task_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_document_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_chunk_id() -> String {
    Uuid::new_v4().to_string()
}
