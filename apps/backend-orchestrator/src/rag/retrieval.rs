use crate::error::RagError;
use crate::models::SearchResult;
use crate::services::MeilisearchService;

pub struct RetrievalService {
    // pub embedding_service: EmbeddingService,
    pub meilisearch_service: MeilisearchService,
}

impl RetrievalService {
    pub fn new() -> Self {
        let meilisearch_service = MeilisearchService::new();

        Self {
            meilisearch_service,
        }
    }

    async fn retrieve_relevant_chunks(
        &self,
        query: &str,
        limit: usize,
        filters: Option<&str>,
    ) -> Result<Vec<SearchResult>, meilisearch_sdk::errors::Error> {
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
    ) -> Result<(Vec<SearchResult>, String), RagError> {
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
