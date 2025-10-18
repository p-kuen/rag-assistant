use crate::error::{RagError, Result};
use crate::models::{ChatResponse, SearchResult};
use crate::services::LlmService;
use futures::StreamExt;

pub struct GenerationService {
    llm_service: LlmService,
}

impl GenerationService {
    pub fn new() -> Result<Self> {
        let llm_service = LlmService::new()?;
        Ok(Self { llm_service })
    }

    pub async fn generate_response_stream(
        &self,
        user_message: &str,
        context: &str,
        sources: Vec<SearchResult>,
    ) -> Result<impl futures::Stream<Item = Result<String>>> {
        let system_prompt = self.build_system_prompt();

        let token_stream = self
            .llm_service
            .generate_response_stream(&system_prompt, user_message, context)
            .await?;

        Ok(token_stream)
    }

    pub async fn generate_response(
        &self,
        user_message: &str,
        context: &str,
        sources: Vec<SearchResult>,
    ) -> Result<ChatResponse> {
        let system_prompt = self.build_system_prompt();

        let response = self
            .llm_service
            .generate_response(&system_prompt, user_message, context)
            .await?;

        Ok(ChatResponse {
            response,
            sources,
            session_id: None,
        })
    }

    fn build_system_prompt(&self) -> String {
        r#"You are a helpful AI assistant that answers questions based on the provided context from a knowledge base. 

Instructions:
1. Use only the information provided in the context to answer questions
2. If the context doesn't contain enough information to answer the question, say so clearly
3. Cite your sources by referencing the source numbers in brackets (e.g., [Source 1], [Source 2])
4. Be concise but comprehensive in your answers
5. If asked about something not in the context, politely explain that you don't have that information
6. Maintain a helpful and professional tone

The context provided will include source information and relevance scores to help you understand the quality of the information."#.to_string()
    }
}
