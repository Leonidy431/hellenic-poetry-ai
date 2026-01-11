/// Модуль обучения языкам.
///
/// Поддерживает обучение:
/// - Древнегреческому языку (κοινή διάλεκτος)
/// - Латыни (lingua Latina)
/// - Церковнославянскому языку
/// - Современному греческому
/// - Русскому языку
/// - Болгарскому языку
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::learning::{LanguageTutor, Language, LanguageLevel};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let tutor = LanguageTutor::new().await?;
///
///     let lesson = tutor
///         .create_lesson()
///         .language(Language::AncientGreek)
///         .level(LanguageLevel::A1)
///         .topic("алфавит")
///         .await?;
///
///     lesson.teach().await?;
///     Ok(())
/// }
/// ```

mod tutor;
mod lesson;
mod exercise;
mod vocabulary;
mod grammar;

pub use tutor::LanguageTutor;
pub use lesson::{Lesson, LessonBuilder};
pub use exercise::{Exercise, ExerciseType, ExerciseResult};
pub use vocabulary::{VocabularyEntry, VocabularyDatabase};
pub use grammar::{GrammarRule, GrammarExplanation};

use serde::{Deserialize, Serialize};

/// Поддерживаемые языки.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// Древнегреческий (κοινή διάλεκτος)
    AncientGreek,

    /// Латынь
    Latin,

    /// Церковнославянский
    ChurchSlavonic,

    /// Современный греческий
    ModernGreek,

    /// Русский
    Russian,

    /// Болгарский
    Bulgarian,
}

impl Language {
    /// Получить название языка на русском.
    pub fn name_ru(&self) -> &'static str {
        match self {
            Language::AncientGreek => "Древнегреческий",
            Language::Latin => "Латынь",
            Language::ChurchSlavonic => "Церковнославянский",
            Language::ModernGreek => "Современный греческий",
            Language::Russian => "Русский",
            Language::Bulgarian => "Болгарский",
        }
    }

    /// Получить название языка на английском.
    pub fn name_en(&self) -> &'static str {
        match self {
            Language::AncientGreek => "Ancient Greek",
            Language::Latin => "Latin",
            Language::ChurchSlavonic => "Church Slavonic",
            Language::ModernGreek => "Modern Greek",
            Language::Russian => "Russian",
            Language::Bulgarian => "Bulgarian",
        }
    }

    /// Получить код языка (ISO 639).
    pub fn code(&self) -> &'static str {
        match self {
            Language::AncientGreek => "grc",
            Language::Latin => "la",
            Language::ChurchSlavonic => "cu",
            Language::ModernGreek => "el",
            Language::Russian => "ru",
            Language::Bulgarian => "bg",
        }
    }
}

/// Уровень владения языком (CEFR).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LanguageLevel {
    /// Начальный уровень
    A1,

    /// Элементарный уровень
    A2,

    /// Средний уровень
    B1,

    /// Средне-продвинутый уровень
    B2,

    /// Продвинутый уровень
    C1,

    /// Профессиональный уровень
    C2,
}

impl LanguageLevel {
    /// Получить описание уровня.
    pub fn description(&self) -> &'static str {
        match self {
            LanguageLevel::A1 => "Начальный: понимание базовых фраз",
            LanguageLevel::A2 => "Элементарный: простые диалоги",
            LanguageLevel::B1 => "Средний: понимание основных текстов",
            LanguageLevel::B2 => "Средне-продвинутый: свободное общение",
            LanguageLevel::C1 => "Продвинутый: сложные тексты",
            LanguageLevel::C2 => "Профессиональный: владение как родным",
        }
    }
}

/// Тип урока.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LessonType {
    /// Изучение алфавита
    Alphabet,

    /// Грамматика
    Grammar,

    /// Лексика
    Vocabulary,

    /// Чтение текстов
    Reading,

    /// Аудирование
    Listening,

    /// Письмо
    Writing,

    /// Разговорная практика
    Speaking,

    /// Перевод
    Translation,

    /// Культура и история
    Culture,
}

impl LessonType {
    /// Получить название типа урока.
    pub fn name(&self) -> &'static str {
        match self {
            LessonType::Alphabet => "Алфавит",
            LessonType::Grammar => "Грамматика",
            LessonType::Vocabulary => "Лексика",
            LessonType::Reading => "Чтение",
            LessonType::Listening => "Аудирование",
            LessonType::Writing => "Письмо",
            LessonType::Speaking => "Разговорная практика",
            LessonType::Translation => "Перевод",
            LessonType::Culture => "Культура и история",
        }
    }
}
