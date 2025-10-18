use crate::error::{RagError, Result};
use async_openai::{
    types::{CreateEmbeddingRequest, EmbeddingInput},
    Client as OpenAiClient,
};
use std::env;

pub struct EmbeddingService {
    client: OpenAiClient<async_openai::config::OpenAIConfig>,
}

impl EmbeddingService {
    pub fn new() -> Result<Self> {
        let embedding_api_url = env::var("EMBEDDING_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());

        let config = async_openai::config::OpenAIConfig::new()
            .with_api_base(embedding_api_url)
            .with_api_key("dummy-key"); // TEI doesn't require real API key

        let client = OpenAiClient::with_config(config);

        Ok(Self { client })
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let request = CreateEmbeddingRequest {
            model: "embeddinggemma-300m".to_string(),
            input: EmbeddingInput::String(text.to_string()),
            encoding_format: None,
            dimensions: None,
            user: None,
        };

        let response = self.client.embeddings().create(request).await?;
        
        if let Some(embedding) = response.data.first() {
            Ok(embedding.embedding.clone())
        } else {
            Err(RagError::EmbeddingFailed("No embedding returned".to_string()))
        }
    }

    pub async fn generate_embeddings_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let request = CreateEmbeddingRequest {
            model: "embeddinggemma-300m".to_string(),
            input: EmbeddingInput::StringArray(texts.clone()),
            encoding_format: None,
            dimensions: None,
            user: None,
        };

        let response = self.client.embeddings().create(request).await?;
        
        let embeddings: Vec<Vec<f32>> = response
            .data
            .into_iter()
            .map(|embedding| embedding.embedding)
            .collect();

        if embeddings.len() != texts.len() {
            return Err(RagError::EmbeddingFailed(
                format!("Expected {} embeddings, got {}", texts.len(), embeddings.len())
            ));
        }

        Ok(embeddings)
    }
}
