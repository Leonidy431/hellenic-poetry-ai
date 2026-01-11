/// Модели данных для AI модуля.
///
/// Определяет структуры для запросов и ответов AI провайдеров.

use serde::{Deserialize, Serialize};

/// Провайдер AI.
///
/// Определяет, какой AI сервис использовать для генерации.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiProvider {
    /// OpenAI GPT-4
    OpenAI,

    /// Anthropic Claude
    Claude,
}

impl AiProvider {
    /// Получить строковое представление провайдера.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::ai::AiProvider;
    ///
    /// assert_eq!(AiProvider::OpenAI.as_str(), "openai");
    /// assert_eq!(AiProvider::Claude.as_str(), "claude");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            AiProvider::OpenAI => "openai",
            AiProvider::Claude => "claude",
        }
    }

    /// Создать провайдера из строки.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::ai::AiProvider;
    ///
    /// let provider = AiProvider::from_str("openai").unwrap();
    /// assert_eq!(provider, AiProvider::OpenAI);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" | "gpt" | "gpt-4" => Some(AiProvider::OpenAI),
            "claude" | "anthropic" => Some(AiProvider::Claude),
            _ => None,
        }
    }
}

/// Запрос к AI.
///
/// Содержит все параметры для генерации текста.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    /// Системный промпт (контекст).
    pub system_prompt: Option<String>,

    /// Пользовательский промпт.
    pub user_prompt: String,

    /// Максимальное количество токенов в ответе.
    pub max_tokens: u32,

    /// Температура генерации (0.0-2.0).
    /// - 0.0 = детерминированный, предсказуемый
    /// - 1.0 = сбалансированный
    /// - 2.0 = креативный, случайный
    pub temperature: f32,

    /// Top-p sampling (0.0-1.0).
    pub top_p: Option<f32>,

    /// Frequency penalty (-2.0 - 2.0).
    pub frequency_penalty: Option<f32>,

    /// Presence penalty (-2.0 - 2.0).
    pub presence_penalty: Option<f32>,
}

impl AiRequest {
    /// Создать новый запрос.
    ///
    /// # Аргументы
    ///
    /// * `prompt` - Пользовательский промпт
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::ai::AiRequest;
    ///
    /// let request = AiRequest::new("Напиши стихотворение о море");
    /// assert_eq!(request.user_prompt, "Напиши стихотворение о море");
    /// ```
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            system_prompt: None,
            user_prompt: prompt.into(),
            max_tokens: 2000,
            temperature: 0.7,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
        }
    }

    /// Установить системный промпт.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::ai::AiRequest;
    ///
    /// let request = AiRequest::new("Напиши стихотворение")
    ///     .with_system("Ты - поэт, пишущий в стиле Пушкина");
    /// ```
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system_prompt = Some(system.into());
        self
    }

    /// Установить максимальное количество токенов.
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Установить температуру.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Установить top-p.
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p.clamp(0.0, 1.0));
        self
    }
}

/// Ответ от AI.
///
/// Содержит сгенерированный текст и метаданные.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    /// Сгенерированный текст.
    pub text: String,

    /// Провайдер, который сгенерировал ответ.
    pub provider: AiProvider,

    /// Количество использованных токенов.
    pub tokens_used: Option<u32>,

    /// Модель, которая использовалась.
    pub model: Option<String>,

    /// Причина остановки генерации.
    pub finish_reason: Option<String>,
}

impl AiResponse {
    /// Создать новый ответ.
    ///
    /// # Аргументы
    ///
    /// * `text` - Сгенерированный текст
    /// * `provider` - Провайдер AI
    pub fn new(text: impl Into<String>, provider: AiProvider) -> Self {
        Self {
            text: text.into(),
            provider,
            tokens_used: None,
            model: None,
            finish_reason: None,
        }
    }

    /// Установить количество использованных токенов.
    pub fn with_tokens(mut self, tokens: u32) -> Self {
        self.tokens_used = Some(tokens);
        self
    }

    /// Установить модель.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Установить причину остановки.
    pub fn with_finish_reason(mut self, reason: impl Into<String>) -> Self {
        self.finish_reason = Some(reason.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_provider() {
        assert_eq!(AiProvider::OpenAI.as_str(), "openai");
        assert_eq!(AiProvider::Claude.as_str(), "claude");

        assert_eq!(AiProvider::from_str("openai"), Some(AiProvider::OpenAI));
        assert_eq!(AiProvider::from_str("GPT-4"), Some(AiProvider::OpenAI));
        assert_eq!(AiProvider::from_str("claude"), Some(AiProvider::Claude));
        assert_eq!(AiProvider::from_str("anthropic"), Some(AiProvider::Claude));
        assert_eq!(AiProvider::from_str("unknown"), None);
    }

    #[test]
    fn test_ai_request() {
        let request = AiRequest::new("Test prompt")
            .with_system("System context")
            .with_max_tokens(1000)
            .with_temperature(0.5);

        assert_eq!(request.user_prompt, "Test prompt");
        assert_eq!(request.system_prompt, Some("System context".to_string()));
        assert_eq!(request.max_tokens, 1000);
        assert_eq!(request.temperature, 0.5);
    }

    #[test]
    fn test_ai_response() {
        let response = AiResponse::new("Generated text", AiProvider::OpenAI)
            .with_tokens(150)
            .with_model("gpt-4")
            .with_finish_reason("stop");

        assert_eq!(response.text, "Generated text");
        assert_eq!(response.provider, AiProvider::OpenAI);
        assert_eq!(response.tokens_used, Some(150));
        assert_eq!(response.model, Some("gpt-4".to_string()));
    }

    #[test]
    fn test_temperature_clamping() {
        let request = AiRequest::new("Test").with_temperature(3.0);
        assert_eq!(request.temperature, 2.0);

        let request = AiRequest::new("Test").with_temperature(-1.0);
        assert_eq!(request.temperature, 0.0);
    }
}
