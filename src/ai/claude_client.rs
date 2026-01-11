/// Anthropic Claude API клиент для Academia Lux.
///
/// Реализует интеграцию с Anthropic Claude для генерации текстов.
/// TODO: Полная реализация будет добавлена позже.

use crate::{Result, Error};
use crate::ai::AiClientTrait;
use async_trait::async_trait;
use tracing::info;

/// Claude API клиент.
pub struct ClaudeClient {
    /// API ключ Anthropic.
    api_key: String,
}

impl ClaudeClient {
    /// Создать новый Claude клиент.
    ///
    /// # Аргументы
    ///
    /// * `api_key` - API ключ Anthropic
    pub async fn new(api_key: &str) -> Result<Self> {
        if api_key.is_empty() {
            return Err(Error::AiApi("Claude API key is empty".to_string()));
        }

        info!("Claude client initialized successfully");

        Ok(Self {
            api_key: api_key.to_string(),
        })
    }
}

#[async_trait]
impl AiClientTrait for ClaudeClient {
    async fn generate(
        &self,
        _prompt: &str,
        _max_tokens: u32,
        _temperature: f32,
    ) -> Result<String> {
        // TODO: Реализовать интеграцию с Claude API
        Err(Error::AiApi(
            "Claude integration not yet implemented".to_string(),
        ))
    }

    async fn generate_with_system(
        &self,
        _system_prompt: &str,
        _user_prompt: &str,
        _max_tokens: u32,
        _temperature: f32,
    ) -> Result<String> {
        // TODO: Реализовать интеграцию с Claude API
        Err(Error::AiApi(
            "Claude integration not yet implemented".to_string(),
        ))
    }

    fn provider_name(&self) -> &str {
        "Claude"
    }
}
