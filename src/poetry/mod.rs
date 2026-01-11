/// Модуль генерации стихов и песен.
///
/// Поддерживает:
/// - Генерацию стихов на русском, греческом, болгарском языках
/// - Различные стихотворные размеры (ямб, хорей, дактиль)
/// - Интеграцию с Suno API для создания музыки
/// - Рифмовку и ритмику
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::poetry::{Generator, PoetryStyle, Language, Meter};
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

pub mod generator;
pub mod rhyme;
pub mod meter;
pub mod styles;
pub mod suno_integration;

pub use generator::Generator;
pub use rhyme::RhymeEngine;
pub use meter::Meter;
pub use styles::PoetryStyle;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Язык стихотворения.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    /// Русский язык.
    Russian,
    /// Греческий язык.
    Greek,
    /// Болгарский язык.
    Bulgarian,
    /// Английский язык.
    English,
}

impl Language {
    /// Получить код языка (ISO 639-1).
    pub fn code(&self) -> &'static str {
        match self {
            Language::Russian => "ru",
            Language::Greek => "el",
            Language::Bulgarian => "bg",
            Language::English => "en",
        }
    }

    /// Получить название языка.
    pub fn name(&self) -> &'static str {
        match self {
            Language::Russian => "Русский",
            Language::Greek => "Ελληνικά",
            Language::Bulgarian => "Български",
            Language::English => "English",
        }
    }
}

/// Стихотворение.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poem {
    /// Уникальный идентификатор.
    pub id: Uuid,

    /// Название стихотворения.
    pub title: String,

    /// Содержание стихотворения.
    pub content: String,

    /// Язык стихотворения.
    pub language: Language,

    /// Стиль стихотворения.
    pub style: PoetryStyle,

    /// Стихотворный размер.
    pub meter: Meter,

    /// ID автора (опционально).
    pub author_id: Option<Uuid>,

    /// Дата создания.
    pub created_at: DateTime<Utc>,

    /// Теги.
    pub tags: Vec<String>,
}

impl Poem {
    /// Создать новое стихотворение.
    pub fn new(
        title: String,
        content: String,
        language: Language,
        style: PoetryStyle,
        meter: Meter,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            language,
            style,
            meter,
            author_id: None,
            created_at: Utc::now(),
            tags: Vec::new(),
        }
    }

    /// Добавить тег.
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Получить количество строк.
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// Получить количество слов.
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code() {
        assert_eq!(Language::Russian.code(), "ru");
        assert_eq!(Language::Greek.code(), "el");
    }

    #[test]
    fn test_poem_creation() {
        let poem = Poem::new(
            "Тестовое стихотворение".to_string(),
            "Строка 1\nСтрока 2\nСтрока 3".to_string(),
            Language::Russian,
            PoetryStyle::Lyric,
            Meter::Iambic,
        );

        assert_eq!(poem.line_count(), 3);
        assert!(poem.word_count() > 0);
    }

    #[test]
    fn test_poem_tags() {
        let mut poem = Poem::new(
            "Тест".to_string(),
            "Содержание".to_string(),
            Language::Russian,
            PoetryStyle::Lyric,
            Meter::Iambic,
        );

        poem.add_tag("православие".to_string());
        poem.add_tag("любовь".to_string());

        assert_eq!(poem.tags.len(), 2);
    }
}
