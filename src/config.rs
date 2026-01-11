/// Модуль конфигурации для Academia Lux.
///
/// Управляет настройками приложения через YAML файлы.
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::config::Config;
///
/// let config = Config::load("config/default.yaml")?;
/// println!("AI Provider: {}", config.ai.provider);
/// # Ok::<(), academia_lux::Error>(())
/// ```

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::Result;

/// Основная конфигурация приложения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Настройки приложения.
    pub app: AppConfig,

    /// Настройки базы данных.
    pub database: DatabaseConfig,

    /// Настройки AI провайдеров.
    pub ai: AiConfig,

    /// Настройки Telegram бота.
    pub telegram: TelegramConfig,

    /// Настройки Яндекс Алисы.
    pub alice: AliceConfig,

    /// Настройки генерации стихов.
    pub poetry: PoetryConfig,

    /// Настройки обучения языкам.
    pub learning: LearningConfig,

    /// Настройки театра.
    pub theater: TheaterConfig,

    /// Настройки православного модуля.
    pub orthodox: OrthodoxConfig,

    /// Настройки логирования.
    pub logging: LoggingConfig,
}

/// Настройки приложения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Название приложения.
    pub name: String,

    /// Версия приложения.
    pub version: String,

    /// Режим работы (development, production).
    pub environment: String,

    /// Порт для web сервера.
    pub port: u16,
}

/// Настройки базы данных.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// URL подключения к базе данных.
    pub url: String,

    /// Максимальное количество соединений.
    pub max_connections: u32,

    /// Таймаут подключения (секунды).
    pub timeout_seconds: u64,
}

/// Настройки AI провайдеров.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// Основной провайдер (openai, anthropic, local).
    pub provider: String,

    /// API ключ OpenAI.
    pub openai_api_key: Option<String>,

    /// API ключ Anthropic Claude.
    pub anthropic_api_key: Option<String>,

    /// API ключ DeepL для переводов.
    pub deepl_api_key: Option<String>,

    /// API ключ Suno для музыки.
    pub suno_api_key: Option<String>,

    /// Модель для генерации текста.
    pub text_model: String,

    /// Модель для генерации изображений.
    pub image_model: String,

    /// Максимальное количество токенов.
    pub max_tokens: u32,

    /// Температура генерации (0.0-2.0).
    pub temperature: f32,
}

/// Настройки Telegram бота.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    /// Токен бота.
    pub bot_token: String,

    /// Webhook URL (опционально).
    pub webhook_url: Option<String>,

    /// Разрешенные пользователи (опционально).
    pub allowed_users: Option<Vec<i64>>,
}

/// Настройки Яндекс Алисы.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliceConfig {
    /// ID навыка Алисы.
    pub skill_id: String,

    /// OAuth токен.
    pub oauth_token: String,

    /// Webhook URL.
    pub webhook_url: String,
}

/// Настройки генерации стихов.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoetryConfig {
    /// Стили стихов по умолчанию.
    pub default_styles: Vec<String>,

    /// Языки для генерации.
    pub languages: Vec<String>,

    /// Максимальная длина стихотворения (строки).
    pub max_lines: u32,

    /// Использовать рифмовку.
    pub use_rhyme: bool,
}

/// Настройки обучения языкам.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Поддерживаемые языки.
    pub supported_languages: Vec<String>,

    /// Уровни сложности (A1-C2).
    pub levels: Vec<String>,

    /// Количество слов в уроке.
    pub words_per_lesson: u32,

    /// Использовать аудио произношение.
    pub use_audio: bool,
}

/// Настройки театра.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheaterConfig {
    /// Жанры постановок.
    pub genres: Vec<String>,

    /// Максимальное количество сцен.
    pub max_scenes: u32,

    /// Время голосования (секунды).
    pub voting_time_seconds: u64,

    /// Минимальное количество голосов.
    pub min_votes: u32,
}

/// Настройки православного модуля.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrthodoxConfig {
    /// Путь к текстам Библии.
    pub bible_path: String,

    /// Путь к святоотеческим текстам.
    pub saints_path: String,

    /// Путь к церковному календарю.
    pub calendar_path: String,

    /// Использовать апологетику.
    pub use_apologetics: bool,
}

/// Настройки логирования.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Уровень логирования (trace, debug, info, warn, error).
    pub level: String,

    /// Формат логов (json, text).
    pub format: String,

    /// Путь к файлу логов (опционально).
    pub file_path: Option<String>,
}

impl Config {
    /// Загрузка конфигурации из YAML файла.
    ///
    /// # Аргументы
    ///
    /// * `path` - Путь к файлу конфигурации
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// use academia_lux::config::Config;
    ///
    /// let config = Config::load("config/default.yaml")?;
    /// # Ok::<(), academia_lux::Error>(())
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Конфигурация по умолчанию.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::config::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.app.name, "Academia Lux");
    /// ```
    pub fn default() -> Self {
        Self {
            app: AppConfig {
                name: "Academia Lux".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                environment: "development".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: "sqlite://academia_lux.db".to_string(),
                max_connections: 10,
                timeout_seconds: 30,
            },
            ai: AiConfig {
                provider: "openai".to_string(),
                openai_api_key: None,
                anthropic_api_key: None,
                deepl_api_key: None,
                suno_api_key: None,
                text_model: "gpt-4".to_string(),
                image_model: "dall-e-3".to_string(),
                max_tokens: 2000,
                temperature: 0.7,
            },
            telegram: TelegramConfig {
                bot_token: "".to_string(),
                webhook_url: None,
                allowed_users: None,
            },
            alice: AliceConfig {
                skill_id: "".to_string(),
                oauth_token: "".to_string(),
                webhook_url: "".to_string(),
            },
            poetry: PoetryConfig {
                default_styles: vec![
                    "lyric".to_string(),
                    "epic".to_string(),
                    "dramatic".to_string(),
                ],
                languages: vec![
                    "russian".to_string(),
                    "greek".to_string(),
                    "bulgarian".to_string(),
                ],
                max_lines: 50,
                use_rhyme: true,
            },
            learning: LearningConfig {
                supported_languages: vec![
                    "greek".to_string(),
                    "russian".to_string(),
                    "bulgarian".to_string(),
                ],
                levels: vec![
                    "A1".to_string(),
                    "A2".to_string(),
                    "B1".to_string(),
                    "B2".to_string(),
                    "C1".to_string(),
                    "C2".to_string(),
                ],
                words_per_lesson: 20,
                use_audio: true,
            },
            theater: TheaterConfig {
                genres: vec![
                    "comedy".to_string(),
                    "drama".to_string(),
                    "fantasy".to_string(),
                    "historical".to_string(),
                ],
                max_scenes: 10,
                voting_time_seconds: 60,
                min_votes: 5,
            },
            orthodox: OrthodoxConfig {
                bible_path: "data/bible_texts".to_string(),
                saints_path: "data/saints_lives".to_string(),
                calendar_path: "data/orthodox_calendar.json".to_string(),
                use_apologetics: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "text".to_string(),
                file_path: None,
            },
        }
    }

    /// Сохранение конфигурации в YAML файл.
    ///
    /// # Аргументы
    ///
    /// * `path` - Путь к файлу для сохранения
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// use academia_lux::config::Config;
    ///
    /// let config = Config::default();
    /// config.save("config/my_config.yaml")?;
    /// # Ok::<(), academia_lux::Error>(())
    /// ```
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.app.name, "Academia Lux");
        assert_eq!(config.app.port, 8080);
        assert_eq!(config.ai.provider, "openai");
    }

    #[test]
    fn test_poetry_config() {
        let config = Config::default();
        assert!(config.poetry.languages.contains(&"greek".to_string()));
        assert!(config.poetry.use_rhyme);
    }

    #[test]
    fn test_learning_config() {
        let config = Config::default();
        assert_eq!(config.learning.words_per_lesson, 20);
        assert!(config.learning.use_audio);
    }
}
