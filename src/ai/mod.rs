/// Модуль AI интеграции для Academia Lux.
///
/// Поддерживает:
/// - OpenAI GPT-4 для генерации текстов
/// - Anthropic Claude для альтернативной генерации
/// - Единый интерфейс для работы с разными провайдерами
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::ai::{AiClient, AiProvider};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = AiClient::new(AiProvider::OpenAI, "your-api-key").await?;
///
///     let response = client
///         .generate("Напиши стихотворение о море")
///         .await?;
///
///     println!("{}", response);
///     Ok(())
/// }
/// ```

pub mod openai_client;
pub mod claude_client;
pub mod prompts;
pub mod models;

pub use openai_client::OpenAiClient;
pub use claude_client::ClaudeClient;
pub use models::{AiRequest, AiResponse, AiProvider};

use crate::{Result, Error};
use async_trait::async_trait;

/// Трейт для AI клиентов.
///
/// Определяет единый интерфейс для работы с разными AI провайдерами.
#[async_trait]
pub trait AiClientTrait: Send + Sync {
    /// Генерация текста на основе промпта.
    ///
    /// # Аргументы
    ///
    /// * `prompt` - Текст промпта для генерации
    /// * `max_tokens` - Максимальное количество токенов в ответе
    /// * `temperature` - Температура генерации (0.0-2.0)
    ///
    /// # Возвращает
    ///
    /// Сгенерированный текст
    async fn generate(
        &self,
        prompt: &str,
        max_tokens: u32,
        temperature: f32,
    ) -> Result<String>;

    /// Генерация с системным промптом.
    ///
    /// # Аргументы
    ///
    /// * `system_prompt` - Системный промпт (контекст)
    /// * `user_prompt` - Пользовательский промпт
    /// * `max_tokens` - Максимальное количество токенов
    /// * `temperature` - Температура генерации
    async fn generate_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        max_tokens: u32,
        temperature: f32,
    ) -> Result<String>;

    /// Получить имя провайдера.
    fn provider_name(&self) -> &str;
}

/// Универсальный AI клиент.
///
/// Автоматически выбирает нужный провайдер на основе конфигурации.
pub struct AiClient {
    /// Внутренний клиент (OpenAI или Claude).
    inner: Box<dyn AiClientTrait>,

    /// Провайдер AI.
    provider: AiProvider,
}

impl AiClient {
    /// Создать новый AI клиент.
    ///
    /// # Аргументы
    ///
    /// * `provider` - Провайдер AI (OpenAI или Claude)
    /// * `api_key` - API ключ
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// use academia_lux::ai::{AiClient, AiProvider};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = AiClient::new(
    ///     AiProvider::OpenAI,
    ///     "sk-..."
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(provider: AiProvider, api_key: &str) -> Result<Self> {
        let inner: Box<dyn AiClientTrait> = match provider {
            AiProvider::OpenAI => {
                Box::new(OpenAiClient::new(api_key).await?)
            }
            AiProvider::Claude => {
                Box::new(ClaudeClient::new(api_key).await?)
            }
        };

        Ok(Self { inner, provider })
    }

    /// Генерация текста.
    ///
    /// # Аргументы
    ///
    /// * `prompt` - Промпт для генерации
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::ai::{AiClient, AiProvider};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let client = AiClient::new(AiProvider::OpenAI, "key").await?;
    /// let text = client.generate("Напиши стихотворение").await?;
    /// println!("{}", text);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        self.inner.generate(prompt, 2000, 0.7).await
    }

    /// Генерация с кастомными параметрами.
    pub async fn generate_custom(
        &self,
        prompt: &str,
        max_tokens: u32,
        temperature: f32,
    ) -> Result<String> {
        self.inner.generate(prompt, max_tokens, temperature).await
    }

    /// Генерация с системным промптом.
    pub async fn generate_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String> {
        self.inner
            .generate_with_system(system_prompt, user_prompt, 2000, 0.7)
            .await
    }

    /// Получить провайдера.
    pub fn provider(&self) -> AiProvider {
        self.provider
    }

    /// Получить имя провайдера.
    pub fn provider_name(&self) -> &str {
        self.inner.provider_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_enum() {
        let provider = AiProvider::OpenAI;
        assert_eq!(provider.as_str(), "openai");

        let provider = AiProvider::Claude;
        assert_eq!(provider.as_str(), "claude");
    }
}
