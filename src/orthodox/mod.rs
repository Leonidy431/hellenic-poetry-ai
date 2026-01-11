/// Православный модуль для Academia Lux.
///
/// Содержит:
/// - Работу с текстами Библии
/// - Жития святых
/// - Высказывания святых отцов
/// - Церковный календарь
/// - Апологетику
///
/// Этот модуль используется для:
/// - Генерации стихов на православную тематику
/// - Развития сюжетов театральных постановок
/// - Обучения и духовного наставничества

pub mod bible;
pub mod saints;
pub mod calendar;
pub mod prayers;
pub mod liturgy;
pub mod apologetics;
pub mod holy_fathers;

pub use bible::BibleModule;
pub use saints::SaintsModule;
pub use calendar::OrthodoxCalendar;
pub use prayers::PrayersModule;
pub use liturgy::LiturgyModule;
pub use apologetics::ApologeticsModule;
pub use holy_fathers::HolyFathersModule;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Высказывание святого отца.
///
/// Используется для:
/// - Вдохновения при создании стихов
/// - Развития сюжета театральных постановок
/// - Духовного наставничества
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolyFatherQuote {
    /// Уникальный идентификатор.
    pub id: Uuid,

    /// Имя святого отца.
    pub saint_name: String,

    /// Текст высказывания.
    pub quote: String,

    /// Тема высказывания.
    pub theme: String,

    /// Источник (книга, глава).
    pub source: Option<String>,

    /// Теги для поиска.
    pub tags: Vec<String>,

    /// Дата добавления.
    pub created_at: DateTime<Utc>,
}

impl HolyFatherQuote {
    /// Создать новое высказывание.
    pub fn new(
        saint_name: impl Into<String>,
        quote: impl Into<String>,
        theme: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            saint_name: saint_name.into(),
            quote: quote.into(),
            theme: theme.into(),
            source: None,
            tags: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Добавить источник.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Добавить теги.
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Событие церковного календаря.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChurchEvent {
    /// Название события.
    pub name: String,

    /// Дата события.
    pub date: String,

    /// Описание.
    pub description: String,

    /// Тип события (праздник, пост, память святого).
    pub event_type: ChurchEventType,

    /// Связанные святые.
    pub related_saints: Vec<String>,

    /// Связанные тексты Писания.
    pub related_scriptures: Vec<String>,
}

/// Тип церковного события.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChurchEventType {
    /// Великий праздник.
    MajorFeast,

    /// Малый праздник.
    MinorFeast,

    /// Пост.
    Fast,

    /// Память святого.
    SaintMemory,

    /// Воскресенье.
    Sunday,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holy_father_quote() {
        let quote = HolyFatherQuote::new(
            "Иоанн Златоуст",
            "Любовь есть корень всех благ",
            "любовь"
        )
        .with_source("Беседы на Евангелие от Матфея")
        .with_tags(vec!["любовь".to_string(), "добродетель".to_string()]);

        assert_eq!(quote.saint_name, "Иоанн Златоуст");
        assert_eq!(quote.theme, "любовь");
        assert_eq!(quote.tags.len(), 2);
    }
}
