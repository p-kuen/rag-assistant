use thiserror::Error;

#[derive(Error, Debug)]
pub enum RagError {
    #[error("Meilisearch error: {0}")]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),

    #[error("OpenAI API error: {0}")]
    OpenAi(#[from] async_openai::error::OpenAIError),

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
