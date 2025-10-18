use crate::error::{RagError, Result};
use async_openai::{
    types::{
        ChatCompletionRequest, ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequest, Role,
    },
    Client as OpenAiClient,
};
use futures::StreamExt;
use std::env;

pub struct LlmService {
    client: OpenAiClient<async_openai::config::OpenAIConfig>,
}

impl LlmService {
    pub fn new() -> Result<Self> {
        let llm_api_url = env::var("LLM_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());

        let config = async_openai::config::OpenAIConfig::new()
            .with_api_base(llm_api_url)
            .with_api_key("dummy-key"); // llama.cpp doesn't require real API key

        let client = OpenAiClient::with_config(config);

        Ok(Self { client })
    }

    pub async fn generate_response_stream(
        &self,
        system_prompt: &str,
        user_message: &str,
        context: &str,
    ) -> Result<impl futures::Stream<Item = Result<String>>> {
        let messages = vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_prompt)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(&format!("Context:\n{}\n\nUser Question: {}", context, user_message))
                .build()?,
        ];

        let request = CreateChatCompletionRequest {
            model: "gemma-2-2b-it".to_string(),
            messages,
            stream: Some(true),
            temperature: Some(0.7),
            max_tokens: Some(2048),
            ..Default::default()
        };

        let stream = self
            .client
            .chat()
            .create_stream(request)
            .await
            .map_err(|e| RagError::LlmFailed(e.to_string()))?;

        let token_stream = stream.map(|chunk_result| {
            match chunk_result {
                Ok(chunk) => {
                    if let Some(choice) = chunk.choices.first() {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                Ok(content.clone())
                            } else {
                                Ok(String::new())
                            }
                        } else {
                            Ok(String::new())
                        }
                    } else {
                        Ok(String::new())
                    }
                }
                Err(e) => Err(RagError::LlmFailed(e.to_string())),
            }
        });

        Ok(token_stream)
    }

    pub async fn generate_response(
        &self,
        system_prompt: &str,
        user_message: &str,
        context: &str,
    ) -> Result<String> {
        let messages = vec![
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(system_prompt)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(&format!("Context:\n{}\n\nUser Question: {}", context, user_message))
                .build()?,
        ];

        let request = CreateChatCompletionRequest {
            model: "gemma-2-2b-it".to_string(),
            messages,
            stream: Some(false),
            temperature: Some(0.7),
            max_tokens: Some(2048),
            ..Default::default()
        };

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| RagError::LlmFailed(e.to_string()))?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                Ok(content.clone())
            } else {
                Err(RagError::LlmFailed("No content in response".to_string()))
            }
        } else {
            Err(RagError::LlmFailed("No choices in response".to_string()))
        }
    }
}
