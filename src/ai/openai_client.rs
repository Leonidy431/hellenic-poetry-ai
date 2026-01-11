/// OpenAI API клиент для Academia Lux.
///
/// Реализует интеграцию с OpenAI GPT-4 для генерации текстов.
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::ai::OpenAiClient;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let client = OpenAiClient::new("sk-...").await?;
///
///     let response = client
///         .generate("Напиши стихотворение о море", 500, 0.7)
///         .await?;
///
///     println!("{}", response);
///     Ok(())
/// }
/// ```

use crate::{Result, Error};
use crate::ai::{AiClientTrait, AiProvider};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};

/// OpenAI API клиент.
pub struct OpenAiClient {
    /// HTTP клиент для запросов.
    client: Client,

    /// API ключ OpenAI.
    api_key: String,

    /// Базовый URL API (по умолчанию https://api.openai.com/v1).
    base_url: String,

    /// Модель для использования (по умолчанию gpt-4).
    model: String,
}

impl OpenAiClient {
    /// Создать новый OpenAI клиент.
    ///
    /// # Аргументы
    ///
    /// * `api_key` - API ключ OpenAI (начинается с "sk-")
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если API ключ пустой или невалидный.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// use academia_lux::ai::OpenAiClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = OpenAiClient::new("sk-...").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(api_key: &str) -> Result<Self> {
        // Валидация API ключа
        if api_key.is_empty() {
            return Err(Error::AiApi("OpenAI API key is empty".to_string()));
        }

        if !api_key.starts_with("sk-") {
            warn!("OpenAI API key doesn't start with 'sk-', this might be invalid");
        }

        // Создание HTTP клиента
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| Error::AiApi(format!("Failed to create HTTP client: {}", e)))?;

        info!("OpenAI client initialized successfully");

        Ok(Self {
            client,
            api_key: api_key.to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
        })
    }

    /// Установить кастомную модель.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::ai::OpenAiClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = OpenAiClient::new("sk-...").await?;
    /// client.set_model("gpt-4-turbo");
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
        info!("OpenAI model changed to: {}", model);
    }

    /// Установить кастомный базовый URL (для прокси или локальных моделей).
    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.to_string();
        info!("OpenAI base URL changed to: {}", base_url);
    }
}

#[async_trait]
impl AiClientTrait for OpenAiClient {
    /// Генерация текста через OpenAI API.
    ///
    /// # Аргументы
    ///
    /// * `prompt` - Текст промпта
    /// * `max_tokens` - Максимальное количество токенов (1-4096)
    /// * `temperature` - Температура генерации (0.0-2.0)
    ///
    /// # Возвращает
    ///
    /// Сгенерированный текст
    ///
    /// # Ошибки
    ///
    /// - `Error::AiApi` - если запрос к API не удался
    /// - `Error::RateLimit` - если превышен лимит запросов
    async fn generate(
        &self,
        prompt: &str,
        max_tokens: u32,
        temperature: f32,
    ) -> Result<String> {
        debug!(
            "Generating text with OpenAI: prompt_len={}, max_tokens={}, temperature={}",
            prompt.len(),
            max_tokens,
            temperature
        );

        // Подготовка запроса
        let request = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![OpenAiMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens,
            temperature,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
        };

        // Отправка запроса
        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::AiApi(format!("OpenAI API request failed: {}", e)))?;

        // Проверка статуса
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(match status.as_u16() {
                429 => Error::RateLimit(format!("OpenAI rate limit exceeded: {}", error_text)),
                401 => Error::Authentication(format!("Invalid OpenAI API key: {}", error_text)),
                _ => Error::AiApi(format!("OpenAI API error ({}): {}", status, error_text)),
            });
        }

        // Парсинг ответа
        let response: OpenAiResponse = response
            .json()
            .await
            .map_err(|e| Error::AiApi(format!("Failed to parse OpenAI response: {}", e)))?;

        // Извлечение текста
        let text = response
            .choices
            .first()
            .and_then(|choice| Some(choice.message.content.clone()))
            .ok_or_else(|| Error::AiApi("No response from OpenAI".to_string()))?;

        info!(
            "OpenAI generation successful: tokens_used={:?}",
            response.usage.map(|u| u.total_tokens)
        );

        Ok(text)
    }

    /// Генерация с системным промптом.
    ///
    /// # Аргументы
    ///
    /// * `system_prompt` - Системный промпт (контекст, роль)
    /// * `user_prompt` - Пользовательский промпт
    /// * `max_tokens` - Максимальное количество токенов
    /// * `temperature` - Температура генерации
    async fn generate_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        max_tokens: u32,
        temperature: f32,
    ) -> Result<String> {
        debug!(
            "Generating with system prompt: system_len={}, user_len={}",
            system_prompt.len(),
            user_prompt.len()
        );

        // Подготовка запроса с системным промптом
        let request = OpenAiRequest {
            model: self.model.clone(),
            messages: vec![
                OpenAiMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                OpenAiMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            max_tokens,
            temperature,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
        };

        // Отправка запроса (аналогично generate)
        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| Error::AiApi(format!("OpenAI API request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(match status.as_u16() {
                429 => Error::RateLimit(format!("OpenAI rate limit exceeded: {}", error_text)),
                401 => Error::Authentication(format!("Invalid OpenAI API key: {}", error_text)),
                _ => Error::AiApi(format!("OpenAI API error ({}): {}", status, error_text)),
            });
        }

        let response: OpenAiResponse = response
            .json()
            .await
            .map_err(|e| Error::AiApi(format!("Failed to parse OpenAI response: {}", e)))?;

        let text = response
            .choices
            .first()
            .and_then(|choice| Some(choice.message.content.clone()))
            .ok_or_else(|| Error::AiApi("No response from OpenAI".to_string()))?;

        info!("OpenAI generation with system prompt successful");

        Ok(text)
    }

    fn provider_name(&self) -> &str {
        "OpenAI"
    }
}

// === Структуры для OpenAI API ===

/// Запрос к OpenAI Chat Completions API.
#[derive(Debug, Serialize)]
struct OpenAiRequest {
    /// Модель для использования (gpt-4, gpt-3.5-turbo и т.д.).
    model: String,

    /// Массив сообщений (system, user, assistant).
    messages: Vec<OpenAiMessage>,

    /// Максимальное количество токенов в ответе.
    max_tokens: u32,

    /// Температура генерации (0.0-2.0).
    temperature: f32,

    /// Top-p sampling (опционально).
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    /// Frequency penalty (опционально).
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,

    /// Presence penalty (опционально).
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
}

/// Сообщение в OpenAI API.
#[derive(Debug, Serialize, Deserialize)]
struct OpenAiMessage {
    /// Роль отправителя (system, user, assistant).
    role: String,

    /// Содержание сообщения.
    content: String,
}

/// Ответ от OpenAI Chat Completions API.
#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    /// ID ответа.
    id: String,

    /// Тип объекта (обычно "chat.completion").
    object: String,

    /// Timestamp создания.
    created: u64,

    /// Модель, которая использовалась.
    model: String,

    /// Массив вариантов ответа.
    choices: Vec<OpenAiChoice>,

    /// Информация об использовании токенов.
    usage: Option<OpenAiUsage>,
}

/// Вариант ответа от OpenAI.
#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    /// Индекс варианта.
    index: u32,

    /// Сообщение с ответом.
    message: OpenAiMessage,

    /// Причина остановки генерации.
    finish_reason: String,
}

/// Информация об использовании токенов.
#[derive(Debug, Deserialize)]
struct OpenAiUsage {
    /// Количество токенов в промпте.
    prompt_tokens: u32,

    /// Количество токенов в ответе.
    completion_tokens: u32,

    /// Общее количество токенов.
    total_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_client_creation() {
        // Тест с пустым ключом
        let result = tokio_test::block_on(OpenAiClient::new(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_model_setting() {
        let mut client = tokio_test::block_on(OpenAiClient::new("sk-test")).unwrap();
        client.set_model("gpt-4-turbo");
        assert_eq!(client.model, "gpt-4-turbo");
    }
}
