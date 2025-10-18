use crate::error::{RagError, Result};
use crate::models::{DocumentChunk, SearchResult};
use meilisearch_sdk::{client::Client, search::SearchQuery};
use serde_json::json;
use std::env;

const INDEX_NAME: &str = "rag_documents";

pub struct MeilisearchService {
    client: Client,
}

impl MeilisearchService {
    pub async fn new() -> Result<Self> {
        let meilisearch_url = env::var("MEILISEARCH_URL")
            .unwrap_or_else(|_| "http://localhost:7700".to_string());
        let meilisearch_api_key = env::var("MEILISEARCH_API_KEY").unwrap_or_default();

        let client = if meilisearch_api_key.is_empty() {
            Client::new(meilisearch_url, None)
        } else {
            Client::new(meilisearch_url, Some(&meilisearch_api_key))
        };

        let service = Self { client };
        service.initialize_index().await?;
        Ok(service)
    }

    async fn initialize_index(&self) -> Result<()> {
        let index = self.client.index(INDEX_NAME);

        // Create index if it doesn't exist
        match index.get().await {
            Ok(_) => {
                tracing::info!("Index '{}' already exists", INDEX_NAME);
            }
            Err(_) => {
                tracing::info!("Creating index '{}'", INDEX_NAME);
                self.client.create_index(INDEX_NAME, Some("id")).await?;
            }
        }

        // Configure index settings for RAG
        let settings = json!({
            "searchableAttributes": [
                "content",
                "title", 
                "metadata",
                "hierarchy_lvl1",
                "hierarchy_lvl2",
                "hierarchy_lvl3"
            ],
            "filterableAttributes": [
                "document_type",
                "author",
                "created_at",
                "updated_at",
                "tags",
                "source_file",
                "chunk_index",
                "hierarchy_lvl1",
                "hierarchy_lvl2",
                "hierarchy_lvl3"
            ],
            "sortableAttributes": [
                "created_at",
                "updated_at",
                "relevance_score",
                "chunk_index"
            ],
            "rankingRules": [
                "words",
                "typo", 
                "proximity",
                "attribute",
                "sort",
                "exactness"
            ],
            "embedders": {
                "default": {
                    "source": "http",
                    "url": "http://embedding-api:8080/embeddings",
                    "apiKey": "",
                    "model": "embeddinggemma-300m",
                    "dimensions": 768,
                    "documentTemplate": "{{doc.hierarchy_lvl1}} {{doc.hierarchy_lvl2}} {{doc.hierarchy_lvl3}} {{doc.content}}"
                }
            }
        });

        index.set_settings(&settings).await?;
        tracing::info!("Index settings configured for RAG system");

        Ok(())
    }

    pub async fn index_documents(&self, chunks: Vec<DocumentChunk>) -> Result<()> {
        let index = self.client.index(INDEX_NAME);
        
        // Convert chunks to Meilisearch documents
        let documents: Vec<serde_json::Value> = chunks
            .into_iter()
            .map(|chunk| {
                json!({
                    "id": chunk.id,
                    "content": chunk.content,
                    "title": chunk.metadata.title,
                    "author": chunk.metadata.author,
                    "document_type": chunk.metadata.document_type,
                    "created_at": chunk.metadata.created_at,
                    "updated_at": chunk.metadata.updated_at,
                    "tags": chunk.metadata.tags,
                    "source_file": chunk.source_file,
                    "chunk_index": chunk.chunk_index,
                    "hierarchy_lvl1": chunk.hierarchy_lvl1,
                    "hierarchy_lvl2": chunk.hierarchy_lvl2,
                    "hierarchy_lvl3": chunk.hierarchy_lvl3,
                    "metadata": chunk.metadata
                })
            })
            .collect();

        index.add_documents(&documents, Some("id")).await?;
        tracing::info!("Indexed {} documents", documents.len());

        Ok(())
    }

    pub async fn hybrid_search(
        &self,
        query: &str,
        limit: usize,
        filters: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        let index = self.client.index(INDEX_NAME);

        let mut search_query = SearchQuery::new(&index)
            .with_query(query)
            .with_limit(limit)
            .with_hybrid_search(meilisearch_sdk::search::HybridSearch {
                semantic_ratio: 0.5,
                embedder: "default".to_string(),
            });

        if let Some(filter) = filters {
            search_query = search_query.with_filter(filter);
        }

        let search_results = search_query.execute().await?;

        let results: Vec<SearchResult> = search_results
            .hits
            .into_iter()
            .map(|hit| {
                let document = hit.result;
                SearchResult {
                    id: document["id"].as_str().unwrap_or("").to_string(),
                    content: document["content"].as_str().unwrap_or("").to_string(),
                    metadata: serde_json::from_value(document["metadata"].clone())
                        .unwrap_or_default(),
                    hierarchy_lvl1: document["hierarchy_lvl1"].as_str().map(|s| s.to_string()),
                    hierarchy_lvl2: document["hierarchy_lvl2"].as_str().map(|s| s.to_string()),
                    hierarchy_lvl3: document["hierarchy_lvl3"].as_str().map(|s| s.to_string()),
                    chunk_index: document["chunk_index"].as_u64().unwrap_or(0) as usize,
                    source_file: document["source_file"].as_str().unwrap_or("").to_string(),
                    score: hit.ranking_score.unwrap_or(0.0),
                }
            })
            .collect();

        tracing::info!("Found {} search results for query: {}", results.len(), query);
        Ok(results)
    }

    pub async fn get_document_stats(&self) -> Result<serde_json::Value> {
        let index = self.client.index(INDEX_NAME);
        let stats = index.get_stats().await?;
        Ok(serde_json::to_value(stats)?)
    }

    pub async fn list_documents(&self) -> Result<Vec<crate::models::DocumentInfo>> {
        let index = self.client.index(INDEX_NAME);
        
        // Get all documents and group by source_file to create document list
        let search_results = SearchQuery::new(&index)
            .with_query("")
            .with_limit(1000)
            .execute()
            .await?;

        let mut document_map: std::collections::HashMap<String, crate::models::DocumentInfo> = 
            std::collections::HashMap::new();

        for hit in search_results.hits {
            let document = hit.result;
            let source_file = document["source_file"].as_str().unwrap_or("unknown");
            let title = document["title"].as_str().unwrap_or(source_file);
            let created_at = document["created_at"].as_str().unwrap_or("");

            let entry = document_map.entry(source_file.to_string()).or_insert_with(|| {
                crate::models::DocumentInfo {
                    id: source_file.to_string(),
                    title: title.to_string(),
                    status: "indexed".to_string(),
                    created_at: created_at.to_string(),
                    chunk_count: 0,
                }
            });

            entry.chunk_count += 1;
        }

        Ok(document_map.into_values().collect())
    }
}
