/// # Academia Lux - Hellenic Poetry AI Platform
///
/// Комплексная AI-платформа для:
/// - Создания стихов и песен (интеграция с Suno App)
/// - Обучения греческому, русскому и болгарскому языкам
/// - Интерактивных театральных постановок
/// - Православного образования и духовного развития
/// - Работы с текстами Библии и святоотеческой литературы
///
/// # Примеры
///
/// ## Генерация стихотворения
///
/// ```no_run
/// use academia_lux::poetry::Generator;
/// use academia_lux::poetry::{PoetryStyle, Language, Meter};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let generator = Generator::new().await?;
///
///     let poem = generator
///         .generate()
///         .theme("море и свобода")
///         .style(PoetryStyle::Lyric)
///         .language(Language::Russian)
///         .meter(Meter::Iambic)
///         .await?;
///
///     println!("{}", poem.content);
///     Ok(())
/// }
/// ```
///
/// ## Урок греческого языка
///
/// ```no_run
/// use academia_lux::learning::LanguageTutor;
/// use academia_lux::learning::LanguageLevel;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let tutor = LanguageTutor::new().await?;
///
///     let lesson = tutor
///         .create_lesson()
///         .level(LanguageLevel::A1)
///         .topic("приветствия")
///         .await?;
///
///     lesson.teach().await?;
///     Ok(())
/// }
/// ```
///
/// ## Интерактивная театральная постановка
///
/// ```no_run
/// use academia_lux::theater::InteractivePlay;
/// use academia_lux::theater::Genre;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut play = InteractivePlay::new()
///         .title("Одиссея: Новое путешествие")
///         .genre(Genre::Fantasy)
///         .await?;
///
///     play.start().await?;
///
///     // Зрители голосуют
///     play.vote("option_a", 150).await?;
///     play.vote("option_b", 200).await?;
///
///     // Продолжение сюжета на основе голосов
///     play.next_scene().await?;
///
///     Ok(())
/// }
/// ```

// Основные модули
pub mod error;
pub mod config;

// Генерация контента
pub mod poetry;
pub mod learning;
pub mod theater;

// Православный модуль
pub mod orthodox;

// AI интеграция
pub mod ai;

// Боты и интеграции
pub mod bot;

// Игровая механика
pub mod game;

// Утилиты
pub mod utils;

// Экспорт основных типов
pub use error::{Error, Result};
pub use config::Config;

/// Версия приложения
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Название приложения
pub const APP_NAME: &str = "Academia Lux";

/// Инициализация логирования.
///
/// # Аргументы
///
/// * `level` - Уровень логирования (trace, debug, info, warn, error)
///
/// # Примеры
///
/// ```
/// use academia_lux::init_logging;
///
/// init_logging("info");
/// ```
pub fn init_logging(level: &str) {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("{} v{} initialized", APP_NAME, VERSION);
}

/// Инициализация приложения.
///
/// Выполняет все необходимые настройки:
/// - Загрузка конфигурации
/// - Инициализация логирования
/// - Подключение к базе данных
/// - Инициализация AI клиентов
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::init;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = init("config/default.yaml").await?;
///     Ok(())
/// }
/// ```
pub async fn init(config_path: &str) -> Result<Config> {
    // Загрузка конфигурации
    let config = Config::load(config_path)?;

    // Инициализация логирования
    init_logging(&config.logging.level);

    tracing::info!("Configuration loaded from: {}", config_path);
    tracing::info!("Database: {}", config.database.url);
    tracing::info!("AI Provider: {}", config.ai.provider);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_app_name() {
        assert_eq!(APP_NAME, "Academia Lux");
    }
}
