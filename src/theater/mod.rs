/// Модуль интерактивного театра.
///
/// Создание интерактивных театральных постановок, где зрители
/// голосуют за развитие сюжета в реальном времени.
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::theater::{InteractivePlay, Genre};
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

mod play;
mod scene;
mod character;
mod character_profile;
mod voting;
mod plot;
mod series;

pub use play::{InteractivePlay, PlayBuilder, PlayStatus};
pub use scene::{Scene, SceneType, SceneTransition};
pub use character::{Character, CharacterRole, CharacterRelationship};
pub use character_profile::{
    CharacterProfile, PsychologyProfile, SpeechProfile, AppearanceProfile,
    Temperament, MoralAlignment, SpeechStyle, SpeechPace, VoiceVolume,
    BodyBuild, Skill, SkillLevel, Motivation, DrivingForce, CharacterType,
};
pub use voting::{VotingSystem, VoteOption, VoteResult};
pub use plot::{PlotGenerator, PlotBranch, PlotOutcome};
pub use series::{Series, Season, Episode, StoryArc, SeriesGenerator, SeriesStatus};

use serde::{Deserialize, Serialize};

/// Жанр театральной постановки.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genre {
    /// Трагедия
    Tragedy,

    /// Комедия
    Comedy,

    /// Драма
    Drama,

    /// Фэнтези
    Fantasy,

    /// Исторический
    Historical,

    /// Мифологический
    Mythological,

    /// Романтический
    Romantic,

    /// Приключенческий
    Adventure,
}

impl Genre {
    pub fn name_ru(&self) -> &'static str {
        match self {
            Genre::Tragedy => "Трагедия",
            Genre::Comedy => "Комедия",
            Genre::Drama => "Драма",
            Genre::Fantasy => "Фэнтези",
            Genre::Historical => "Исторический",
            Genre::Mythological => "Мифологический",
            Genre::Romantic => "Романтический",
            Genre::Adventure => "Приключенческий",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Genre::Tragedy => "Серьёзная постановка с трагическим финалом",
            Genre::Comedy => "Лёгкая и весёлая постановка",
            Genre::Drama => "Драматическая история с глубокими переживаниями",
            Genre::Fantasy => "Фантастическая история с магией и приключениями",
            Genre::Historical => "Историческая постановка на основе реальных событий",
            Genre::Mythological => "История на основе древних мифов",
            Genre::Romantic => "Романтическая история о любви",
            Genre::Adventure => "Приключенческая история с экшеном",
        }
    }
}

/// Настроение сцены.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mood {
    /// Радостное
    Joyful,

    /// Грустное
    Sad,

    /// Напряжённое
    Tense,

    /// Спокойное
    Calm,

    /// Таинственное
    Mysterious,

    /// Романтическое
    Romantic,

    /// Драматическое
    Dramatic,
}

impl Mood {
    pub fn name_ru(&self) -> &'static str {
        match self {
            Mood::Joyful => "Радостное",
            Mood::Sad => "Грустное",
            Mood::Tense => "Напряжённое",
            Mood::Calm => "Спокойное",
            Mood::Mysterious => "Таинственное",
            Mood::Romantic => "Романтическое",
            Mood::Dramatic => "Драматическое",
        }
    }
}
