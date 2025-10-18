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
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid file type: {0}")]
    InvalidFileType(String),
    
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Indexing failed: {0}")]
    IndexingFailed(String),
    
    #[error("Chunking failed: {0}")]
    ChunkingFailed(String),
    
    #[error("Embedding generation failed: {0}")]
    EmbeddingFailed(String),
    
    #[error("LLM generation failed: {0}")]
    LlmFailed(String),
}

pub type Result<T> = std::result::Result<T, RagError>;
