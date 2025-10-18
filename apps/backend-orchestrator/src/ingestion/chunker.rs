use crate::error::{RagError, Result};
use crate::models::{Document, DocumentChunk, DocumentMetadata};
use text_splitter::{TextSplitter, ChunkConfig};

pub struct DocumentChunker {
    chunk_size: usize,
    chunk_overlap: usize,
}

impl DocumentChunker {
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
        }
    }

    pub fn default() -> Self {
        Self::new(512, 50) // 512 tokens with 50 token overlap
    }

    pub fn chunk_document(&self, document: &Document) -> Result<Vec<DocumentChunk>> {
        let mut chunks = Vec::new();
        
        // Create text splitter with token-based chunking
        let splitter = TextSplitter::new(
            ChunkConfig::new(self.chunk_size)
                .with_overlap(self.chunk_overlap)
                .with_sizing(text_splitter::TextSizing::default()),
        );

        // Split the content into chunks
        let text_chunks = splitter
            .chunks(&document.content)
            .map_err(|e| RagError::ChunkingFailed(e.to_string()))?;

        for (index, chunk_text) in text_chunks.iter().enumerate() {
            let chunk = DocumentChunk {
                id: crate::models::generate_chunk_id(),
                content: chunk_text.to_string(),
                metadata: document.metadata.clone(),
                hierarchy_lvl1: self.extract_hierarchy_level(chunk_text, 1),
                hierarchy_lvl2: self.extract_hierarchy_level(chunk_text, 2),
                hierarchy_lvl3: self.extract_hierarchy_level(chunk_text, 3),
                chunk_index: index,
                source_file: document.title.clone(),
                embedding: None, // Will be populated by embedding service
            };

            chunks.push(chunk);
        }

        tracing::info!(
            "Created {} chunks for document '{}'",
            chunks.len(),
            document.title
        );

        Ok(chunks)
    }

    fn extract_hierarchy_level(&self, content: &str, level: usize) -> Option<String> {
        let prefix = "#".repeat(level);
        let search_pattern = format!("{} ", prefix);

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(&search_pattern) {
                return Some(trimmed[search_pattern.len()..].to_string());
            }
        }

        // If no heading found at this level, look for the closest higher level heading
        if level > 1 {
            for i in (1..level).rev() {
                if let Some(heading) = self.extract_hierarchy_level(content, i) {
                    return Some(heading);
                }
            }
        }

        None
    }

    pub fn chunk_text(&self, text: &str, source: &str) -> Result<Vec<DocumentChunk>> {
        let mut chunks = Vec::new();
        
        let splitter = TextSplitter::new(
            ChunkConfig::new(self.chunk_size)
                .with_overlap(self.chunk_overlap)
                .with_sizing(text_splitter::TextSizing::default()),
        );

        let text_chunks = splitter
            .chunks(text)
            .map_err(|e| RagError::ChunkingFailed(e.to_string()))?;

        for (index, chunk_text) in text_chunks.iter().enumerate() {
            let chunk = DocumentChunk {
                id: crate::models::generate_chunk_id(),
                content: chunk_text.to_string(),
                metadata: DocumentMetadata {
                    title: Some(source.to_string()),
                    author: None,
                    tags: None,
                    document_type: Some("text".to_string()),
                    created_at: Some(chrono::Utc::now().to_rfc3339()),
                    updated_at: Some(chrono::Utc::now().to_rfc3339()),
                },
                hierarchy_lvl1: None,
                hierarchy_lvl2: None,
                hierarchy_lvl3: None,
                chunk_index: index,
                source_file: source.to_string(),
                embedding: None,
            };

            chunks.push(chunk);
        }

        Ok(chunks)
    }
}
