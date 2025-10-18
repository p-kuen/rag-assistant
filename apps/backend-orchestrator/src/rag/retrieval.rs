use crate::error::{RagError, Result};
use crate::models::SearchResult;
use crate::services::{EmbeddingService, MeilisearchService};

pub struct RetrievalService {
    pub embedding_service: EmbeddingService,
    pub meilisearch_service: MeilisearchService,
}

impl RetrievalService {
    pub fn new() -> Result<Self> {
        let embedding_service = EmbeddingService::new()?;
        let meilisearch_service = futures::executor::block_on(MeilisearchService::new())?;

        Ok(Self {
            embedding_service,
            meilisearch_service,
        })
    }

    pub async fn retrieve_relevant_chunks(
        &self,
        query: &str,
        limit: usize,
        filters: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        // Generate embedding for the query
        let _query_embedding = self.embedding_service.generate_embedding(query).await?;

        // Perform hybrid search in Meilisearch
        let results = self
            .meilisearch_service
            .hybrid_search(query, limit, filters)
            .await?;

        tracing::info!(
            "Retrieved {} relevant chunks for query: '{}'",
            results.len(),
            query
        );

        Ok(results)
    }

    pub async fn retrieve_with_context(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<(Vec<SearchResult>, String)> {
        let results = self.retrieve_relevant_chunks(query, limit, None).await?;

        // Build context from retrieved chunks
        let context = self.build_context(&results);

        Ok((results, context))
    }

    fn build_context(&self, results: &[SearchResult]) -> String {
        let mut context_parts = Vec::new();

        for (i, result) in results.iter().enumerate() {
            let mut chunk_context = format!("[Source {}: {}]\n", i + 1, result.source_file);
            
            if let Some(h1) = &result.hierarchy_lvl1 {
                chunk_context.push_str(&format!("Section: {}\n", h1));
            }
            if let Some(h2) = &result.hierarchy_lvl2 {
                chunk_context.push_str(&format!("Subsection: {}\n", h2));
            }
            if let Some(h3) = &result.hierarchy_lvl3 {
                chunk_context.push_str(&format!("Sub-subsection: {}\n", h3));
            }

            chunk_context.push_str(&format!("Content: {}\n", result.content));
            chunk_context.push_str(&format!("Relevance Score: {:.3}\n", result.score));
            chunk_context.push_str("\n---\n\n");

            context_parts.push(chunk_context);
        }

        context_parts.join("")
    }
}
