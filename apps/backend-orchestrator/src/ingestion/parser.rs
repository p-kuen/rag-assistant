use crate::models::{Document, DocumentMetadata};
use pulldown_cmark::{html, Options, Parser};
use std::collections::HashMap;

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse_document(content: &str, filename: &str) -> Result<Document> {
        let (frontmatter, markdown_content) = Self::extract_frontmatter(content)?;
        let metadata = Self::parse_metadata(frontmatter, filename)?;
        let html_content = Self::markdown_to_html(&markdown_content)?;
        let hierarchy = Self::extract_hierarchy(&markdown_content)?;

        let document = Document {
            id: crate::models::generate_document_id(),
            title: metadata.title.clone().unwrap_or_else(|| {
                Self::extract_title_from_content(&markdown_content)
                    .unwrap_or_else(|| filename.to_string())
            }),
            content: html_content,
            metadata,
            chunks: vec![], // Will be populated by chunker
        };

        Ok(document)
    }

    fn extract_frontmatter(content: &str) -> Result<(Option<String>, String)> {
        if content.starts_with("---\n") {
            if let Some(end_pos) = content.find("\n---\n") {
                let frontmatter = content[4..end_pos].to_string();
                let markdown_content = content[end_pos + 5..].to_string();
                return Ok((Some(frontmatter), markdown_content));
            }
        }
        Ok((None, content.to_string()))
    }

    fn parse_metadata(frontmatter: Option<String>, filename: &str) -> Result<DocumentMetadata> {
        let mut metadata = DocumentMetadata {
            title: None,
            author: None,
            tags: None,
            document_type: Some("markdown".to_string()),
            created_at: Some(chrono::Utc::now().to_rfc3339()),
            updated_at: Some(chrono::Utc::now().to_rfc3339()),
        };

        if let Some(frontmatter_content) = frontmatter {
            match serde_yaml::from_str::<HashMap<String, serde_yaml::Value>>(&frontmatter_content) {
                Ok(frontmatter_data) => {
                    if let Some(title) = frontmatter_data.get("title") {
                        metadata.title = title.as_str().map(|s| s.to_string());
                    }
                    if let Some(author) = frontmatter_data.get("author") {
                        metadata.author = author.as_str().map(|s| s.to_string());
                    }
                    if let Some(tags) = frontmatter_data.get("tags") {
                        if let Some(tag_array) = tags.as_sequence() {
                            metadata.tags = Some(
                                tag_array
                                    .iter()
                                    .filter_map(|tag| tag.as_str().map(|s| s.to_string()))
                                    .collect(),
                            );
                        }
                    }
                    if let Some(doc_type) = frontmatter_data.get("type") {
                        metadata.document_type = doc_type.as_str().map(|s| s.to_string());
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to parse frontmatter: {}", e);
                }
            }
        }

        // If no title in frontmatter, try to extract from filename
        if metadata.title.is_none() {
            metadata.title = Some(
                filename
                    .replace(".md", "")
                    .replace("_", " ")
                    .replace("-", " "),
            );
        }

        Ok(metadata)
    }

    fn markdown_to_html(markdown: &str) -> Result<String> {
        let parser = Parser::new_ext(markdown, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }

    fn extract_hierarchy(markdown: &str) -> Result<HashMap<String, Vec<String>>> {
        let mut hierarchy: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_h1: Option<String> = None;
        let mut current_h2: Option<String> = None;
        let mut current_h3: Option<String> = None;

        for line in markdown.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                current_h1 = Some(trimmed[2..].to_string());
                current_h2 = None;
                current_h3 = None;
            } else if trimmed.starts_with("## ") {
                current_h2 = Some(trimmed[3..].to_string());
                current_h3 = None;
            } else if trimmed.starts_with("### ") {
                current_h3 = Some(trimmed[4..].to_string());
            }

            if let Some(h1) = &current_h1 {
                hierarchy.entry(h1.clone()).or_insert_with(Vec::new);
                if let Some(h2) = &current_h2 {
                    hierarchy.entry(h2.clone()).or_insert_with(Vec::new);
                    if let Some(h3) = &current_h3 {
                        hierarchy.entry(h3.clone()).or_insert_with(Vec::new);
                    }
                }
            }
        }

        Ok(hierarchy)
    }

    fn extract_title_from_content(content: &str) -> Option<String> {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return Some(trimmed[2..].to_string());
            }
        }
        None
    }
}
