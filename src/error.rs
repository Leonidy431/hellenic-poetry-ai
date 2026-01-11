/// Модуль обработки ошибок для Academia Lux.
///
/// Определяет типы ошибок для всех компонентов системы.
/// Следует принципам:
/// - Явная обработка ошибок (Result<T, Error>)
/// - Информативные сообщения об ошибках
/// - Конвертация между типами ошибок
///
/// # Примеры
///
/// ```
/// use academia_lux::error::{Error, Result};
///
/// fn example() -> Result<()> {
///     Err(Error::PoetryGeneration("Failed to generate poem".to_string()))
/// }
/// ```

use thiserror::Error;

/// Основной тип Result для всего приложения.
pub type Result<T> = std::result::Result<T, Error>;

/// Перечисление всех возможных ошибок в системе.
#[derive(Error, Debug)]
pub enum Error {
    // === AI и генерация контента ===
    /// Ошибки при работе с AI API.
    #[error("AI API error: {0}")]
    AiApi(String),

    /// Ошибки генерации стихов.
    #[error("Poetry generation error: {0}")]
    PoetryGeneration(String),

    /// Ошибки генерации музыки (Suno API).
    #[error("Music generation error: {0}")]
    MusicGeneration(String),

    // === Обучение языкам ===
    /// Ошибки модуля обучения языкам.
    #[error("Language learning error: {0}")]
    LanguageLearning(String),

    /// Ошибки перевода (DeepL).
    #[error("Translation error: {0}")]
    Translation(String),

    // === Театр и интерактив ===
    /// Ошибки театрального модуля.
    #[error("Theater error: {0}")]
    Theater(String),

    /// Ошибки голосования.
    #[error("Voting error: {0}")]
    Voting(String),

    // === Православный модуль ===
    /// Ошибки работы с православными текстами.
    #[error("Orthodox module error: {0}")]
    Orthodox(String),

    /// Ошибки апологетики.
    #[error("Apologetics error: {0}")]
    Apologetics(String),

    // === Боты и интеграции ===
    /// Ошибки Telegram бота.
    #[error("Telegram bot error: {0}")]
    TelegramBot(String),

    /// Ошибки интеграции с Алисой.
    #[error("Yandex Alice error: {0}")]
    YandexAlice(String),

    // === Система ===
    /// Ошибки конфигурации.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Ошибки базы данных.
    #[error("Database error: {0}")]
    Database(String),

    /// Ошибки валидации входных данных.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Ошибки сериализации/десериализации.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Ошибки ввода-вывода.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Ошибки HTTP запросов.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Ошибки парсинга JSON.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Ошибки парсинга YAML.
    #[error("YAML error: {0}")]
    Yaml(String),

    /// Ошибки работы с UUID.
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    /// Ошибки rate limiting.
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Ошибки аутентификации.
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Ресурс не найден.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Неизвестная ошибка.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Конвертация из serde_yaml::Error
impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::Yaml(err.to_string())
    }
}

// Конвертация из anyhow::Error
impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Unknown(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::PoetryGeneration("Failed to generate poem".to_string());
        assert_eq!(
            err.to_string(),
            "Poetry generation error: Failed to generate poem"
        );
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }

    #[test]
    fn test_orthodox_error() {
        let err = Error::Orthodox("Bible text not found".to_string());
        assert!(err.to_string().contains("Orthodox module error"));
    }

    #[test]
    fn test_telegram_error() {
        let err = Error::TelegramBot("Failed to send message".to_string());
        assert!(err.to_string().contains("Telegram bot error"));
    }
}
