use crate::error::{RagError, Result};
use crate::models::{Document, DocumentChunk, TaskStatus, TaskStatusType, TaskStore};
use crate::services::MeilisearchService;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct IngestionPipeline {
    chunker: crate::ingestion::chunker::DocumentChunker,
    meilisearch_service: MeilisearchService,
    task_store: TaskStore,
}

impl IngestionPipeline {
    pub async fn new(task_store: TaskStore) -> Result<Self> {
        let chunker = crate::ingestion::chunker::DocumentChunker::default();
        let meilisearch_service = MeilisearchService::new().await?;

        Ok(Self {
            chunker,
            meilisearch_service,
            task_store,
        })
    }

    pub async fn process_document(&self, document: Document, task_id: String) -> Result<()> {
        // Update task status to processing
        self.update_task_status(&task_id, TaskStatusType::Processing, None, None)
            .await;

        // Step 1: Chunk the document
        let chunks = self.chunker.chunk_document(&document).map_err(|e| {
            self.update_task_status(&task_id, TaskStatusType::Failed, None, Some(e.to_string()));
            e
        })?;

        tracing::info!(
            "Document '{}' chunked into {} pieces",
            document.title,
            chunks.len()
        );

        // Step 2: Generate embeddings for each chunk
        let mut chunks_with_embeddings = Vec::new();
        for (i, mut chunk) in chunks.into_iter().enumerate() {
            chunks_with_embeddings.push(chunk);

            // Update progress
            let progress = Some((i as f32 + 1.0) / chunks_with_embeddings.len() as f32 * 0.7); // 70% for embedding
            self.update_task_status(&task_id, TaskStatusType::Processing, progress, None)
                .await;
        }

        tracing::info!(
            "Generated embeddings for {} chunks",
            chunks_with_embeddings.len()
        );

        // Step 3: Index documents in Meilisearch
        self.meilisearch_service
            .index_documents(chunks_with_embeddings)
            .await
            .map_err(|e| {
                self.update_task_status(
                    &task_id,
                    TaskStatusType::Failed,
                    None,
                    Some(e.to_string()),
                );
                e
            })?;

        // Update task status to completed
        self.update_task_status(&task_id, TaskStatusType::Succeeded, Some(1.0), None)
            .await;

        tracing::info!("Document '{}' successfully indexed", document.title);
        Ok(())
    }

    pub async fn process_text(
        &self,
        content: String,
        title: String,
        task_id: String,
    ) -> Result<()> {
        // Update task status to processing
        self.update_task_status(&task_id, TaskStatusType::Processing, None, None)
            .await;

        // Step 1: Chunk the text
        let chunks = self.chunker.chunk_text(&content, &title).map_err(|e| {
            self.update_task_status(&task_id, TaskStatusType::Failed, None, Some(e.to_string()));
            e
        })?;

        tracing::info!("Text '{}' chunked into {} pieces", title, chunks.len());

        // Step 2: Generate embeddings for each chunk
        let mut chunks_with_embeddings = Vec::new();
        for (i, mut chunk) in chunks.into_iter().enumerate() {
            let embedding = self
                .embedding_service
                .generate_embedding(&chunk.content)
                .await
                .map_err(|e| {
                    self.update_task_status(
                        &task_id,
                        TaskStatusType::Failed,
                        None,
                        Some(e.to_string()),
                    );
                    e
                })?;

            chunk.embedding = Some(embedding);
            chunks_with_embeddings.push(chunk);

            // Update progress
            let progress = Some((i as f32 + 1.0) / chunks_with_embeddings.len() as f32 * 0.7); // 70% for embedding
            self.update_task_status(&task_id, TaskStatusType::Processing, progress, None)
                .await;
        }

        tracing::info!(
            "Generated embeddings for {} chunks",
            chunks_with_embeddings.len()
        );

        // Step 3: Index documents in Meilisearch
        self.meilisearch_service
            .index_documents(chunks_with_embeddings)
            .await
            .map_err(|e| {
                self.update_task_status(
                    &task_id,
                    TaskStatusType::Failed,
                    None,
                    Some(e.to_string()),
                );
                e
            })?;

        // Update task status to completed
        self.update_task_status(&task_id, TaskStatusType::Succeeded, Some(1.0), None)
            .await;

        tracing::info!("Text '{}' successfully indexed", title);
        Ok(())
    }

    async fn update_task_status(
        &self,
        task_id: &str,
        status: TaskStatusType,
        progress: Option<f32>,
        error: Option<String>,
    ) {
        let mut tasks = self.task_store.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = status;
            task.progress = progress;
            task.error = error;
            task.updated_at = chrono::Utc::now();
        }
    }

    pub async fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        let tasks = self.task_store.read().await;
        tasks.get(task_id).cloned()
    }

    pub async fn create_task(&self, task_id: String) {
        let task = TaskStatus {
            id: task_id.clone(),
            status: TaskStatusType::Pending,
            progress: Some(0.0),
            error: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let mut tasks = self.task_store.write().await;
        tasks.insert(task_id, task);
    }
}
