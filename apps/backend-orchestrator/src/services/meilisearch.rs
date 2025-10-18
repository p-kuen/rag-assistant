use crate::models::{self, InputDocument};
use meilisearch_sdk::settings::{Embedder, EmbedderSource, Settings};
use meilisearch_sdk::{client::Client, search::SearchQuery};
use std::collections::HashMap;
use std::env;

const INDEX_NAME: &str = "rag_documents";

pub struct MeilisearchService {
    client: Client,
}

impl MeilisearchService {
    pub async fn new() -> Result<MeilisearchService, meilisearch_sdk::errors::Error> {
        let meilisearch_url =
            env::var("MEILISEARCH_URL").unwrap_or_else(|_| "http://localhost:7700".to_string());
        let meilisearch_api_key = env::var("MEILISEARCH_API_KEY").ok();

        let client = Client::new(meilisearch_url, meilisearch_api_key)?;

        let service = Self { client };
        service.initialize_index().await?;

        Ok(service)
    }

    async fn initialize_index(&self) -> Result<(), meilisearch_sdk::errors::Error> {
        let index = self.client.get_index(INDEX_NAME).await;

        // Create index if it doesn't exist
        let index = match index {
            Ok(index) => {
                tracing::info!("Index '{}' already exists", INDEX_NAME);
                index
            }
            Err(_) => {
                tracing::info!("Creating index '{}'", INDEX_NAME);
                let task = self.client.create_index(INDEX_NAME, Some("id")).await?;

                let task = task
                    .wait_for_completion(&self.client, None, None)
                    .await
                    .unwrap();

                task.try_make_index(&self.client).unwrap()
            }
        };

        // Configure index settings for RAG
        let searchable_attributes = vec![
            "content".to_owned(),
            "title".to_owned(),
            "metadata".to_owned(),
            "hierarchy_lvl1".to_owned(),
            "hierarchy_lvl2".to_owned(),
            "hierarchy_lvl3".to_owned(),
        ];

        let filterable_attributes = vec![
            "document_type".to_owned(),
            "author".to_owned(),
            "created_at".to_owned(),
            "updated_at".to_owned(),
            "tags".to_owned(),
            "source_file".to_owned(),
            "chunk_index".to_owned(),
            "hierarchy_lvl1".to_owned(),
            "hierarchy_lvl2".to_owned(),
            "hierarchy_lvl3".to_owned(),
        ];

        let sortable_attributes = vec![
            "created_at".to_owned(),
            "updated_at".to_owned(),
            "relevance_score".to_owned(),
            "chunk_index".to_owned(),
        ];

        let ranking_rules = vec![
            "words".to_owned(),
            "typo".to_owned(),
            "proximity".to_owned(),
            "attribute".to_owned(),
            "sort".to_owned(),
            "exactness".to_owned(),
        ];

        let embedders: HashMap<_, _> = HashMap::from([(
            "default",
            Embedder {
                source: EmbedderSource::Ollama,
                url: Some("http://embedding-api:11434/api/embed".to_owned()),
                api_key: None,
                model: Some("embeddinggemma:300m".to_owned()),
                revision: None,
                pooling: None,
                document_template: Some("A document with the content {{doc.content}}".to_owned()),
                document_template_max_bytes: None,
                dimensions: Some(768),
                distribution: None,
                request: None,
                response: None,
                binary_quantized: None,
                indexing_embedder: None,
                search_embedder: None,
            },
        )]);

        let settings = Settings::new()
            .with_searchable_attributes(searchable_attributes)
            .with_filterable_attributes(filterable_attributes)
            .with_sortable_attributes(sortable_attributes)
            .with_ranking_rules(ranking_rules)
            .with_embedders(embedders);

        index.set_settings(&settings).await?;
        tracing::info!("Index settings configured for RAG system");

        Ok(())
    }

    pub async fn index_documents(
        &self,
        documents: Vec<InputDocument>,
    ) -> Result<(), meilisearch_sdk::errors::Error> {
        let index = self.client.index(INDEX_NAME);

        index.add_documents(&documents, Some("id")).await?;
        tracing::info!("Indexed {} documents", documents.len());

        Ok(())
    }

    pub async fn hybrid_search(
        &self,
        query: &str,
        limit: usize,
        filters: Option<&str>,
    ) -> Result<Vec<models::SearchResult>, meilisearch_sdk::errors::Error> {
        let index = self.client.index(INDEX_NAME);

        let mut search_query = SearchQuery::new(&index)
            .with_query(query)
            .with_limit(limit)
            .with_hybrid("default", 0.5)
            .build();

        if let Some(filter) = filters {
            search_query = search_query.with_filter(filter).build();
        }

        let search_results = search_query.execute::<models::SearchResult>().await?;

        tracing::info!(
            "Found {} search results for query: {}",
            search_results.hits.len(),
            query
        );
        Ok(search_results
            .hits
            .iter()
            .map(|hit| hit.result.to_owned())
            .collect())
    }

    pub async fn list_documents(
        &self,
    ) -> Result<Vec<crate::models::DocumentInfo>, meilisearch_sdk::errors::Error> {
        let index = self.client.index(INDEX_NAME);

        // Get all documents and group by source_file to create document list
        let search_results = SearchQuery::new(&index)
            .with_query("")
            .with_limit(1000)
            .execute::<models::SearchResult>()
            .await?;

        Ok(search_results
            .hits
            .iter()
            .map(|hit| crate::models::DocumentInfo {
                id: hit.result.id.to_owned(),
                title: hit.result.title.to_owned(),
                status: "indexed".to_owned(),
                created_at: hit
                    .result
                    .metadata
                    .created_at
                    .to_owned()
                    .unwrap_or_default(),
            })
            .collect())
    }
}
