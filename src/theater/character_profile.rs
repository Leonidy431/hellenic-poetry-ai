/// Расширенный профиль персонажа с психологическими характеристиками.

use crate::theater::{Character, CharacterRole};
use serde::{Deserialize, Serialize};

/// Расширенный профиль персонажа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProfile {
    /// Базовая информация
    pub character: Character,

    /// Психологические характеристики
    pub psychology: PsychologyProfile,

    /// Речевые особенности
    pub speech: SpeechProfile,

    /// Внешность
    pub appearance: AppearanceProfile,

    /// Навыки и способности
    pub skills: Vec<Skill>,

    /// Мотивация
    pub motivation: Motivation,

    /// Страхи и слабости
    pub fears: Vec<String>,

    /// Цели
    pub goals: Vec<String>,
}

impl CharacterProfile {
    pub fn new(character: Character) -> Self {
        Self {
            character,
            psychology: PsychologyProfile::default(),
            speech: SpeechProfile::default(),
            appearance: AppearanceProfile::default(),
            skills: vec![],
            motivation: Motivation::default(),
            fears: vec![],
            goals: vec![],
        }
    }

    pub fn with_psychology(mut self, psychology: PsychologyProfile) -> Self {
        self.psychology = psychology;
        self
    }

    pub fn with_speech(mut self, speech: SpeechProfile) -> Self {
        self.speech = speech;
        self
    }

    pub fn with_appearance(mut self, appearance: AppearanceProfile) -> Self {
        self.appearance = appearance;
        self
    }
}

/// Психологический профиль персонажа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologyProfile {
    /// Тип личности (MBTI)
    pub mbti_type: Option<String>,

    /// Темперамент
    pub temperament: Temperament,

    /// Черты характера
    pub traits: Vec<PersonalityTrait>,

    /// Эмоциональная стабильность (1-10)
    pub emotional_stability: u8,

    /// Интеллект (1-10)
    pub intelligence: u8,

    /// Харизма (1-10)
    pub charisma: u8,

    /// Моральный компас
    pub moral_alignment: MoralAlignment,

    /// Психологические особенности
    pub quirks: Vec<String>,
}

impl Default for PsychologyProfile {
    fn default() -> Self {
        Self {
            mbti_type: None,
            temperament: Temperament::Balanced,
            traits: vec![],
            emotional_stability: 5,
            intelligence: 5,
            charisma: 5,
            moral_alignment: MoralAlignment::Neutral,
            quirks: vec![],
        }
    }
}

/// Темперамент.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Temperament {
    /// Холерик (энергичный, вспыльчивый)
    Choleric,

    /// Сангвиник (оптимистичный, общительный)
    Sanguine,

    /// Флегматик (спокойный, уравновешенный)
    Phlegmatic,

    /// Меланхолик (чувствительный, задумчивый)
    Melancholic,

    /// Сбалансированный
    Balanced,
}

impl Temperament {
    pub fn name_ru(&self) -> &'static str {
        match self {
            Temperament::Choleric => "Холерик",
            Temperament::Sanguine => "Сангвиник",
            Temperament::Phlegmatic => "Флегматик",
            Temperament::Melancholic => "Меланхолик",
            Temperament::Balanced => "Сбалансированный",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Temperament::Choleric => "Энергичный, решительный, вспыльчивый",
            Temperament::Sanguine => "Оптимистичный, общительный, импульсивный",
            Temperament::Phlegmatic => "Спокойный, уравновешенный, медлительный",
            Temperament::Melancholic => "Чувствительный, задумчивый, перфекционист",
            Temperament::Balanced => "Сбалансированный темперамент",
        }
    }
}

/// Черта характера.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrait {
    pub name: String,
    pub intensity: u8, // 1-10
    pub description: String,
}

/// Моральное мировоззрение.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoralAlignment {
    /// Законопослушный добрый
    LawfulGood,

    /// Нейтральный добрый
    NeutralGood,

    /// Хаотичный добрый
    ChaoticGood,

    /// Законопослушный нейтральный
    LawfulNeutral,

    /// Истинно нейтральный
    Neutral,

    /// Хаотичный нейтральный
    ChaoticNeutral,

    /// Законопослушный злой
    LawfulEvil,

    /// Нейтральный злой
    NeutralEvil,

    /// Хаотичный злой
    ChaoticEvil,
}

impl MoralAlignment {
    pub fn name_ru(&self) -> &'static str {
        match self {
            MoralAlignment::LawfulGood => "Законопослушный добрый",
            MoralAlignment::NeutralGood => "Нейтральный добрый",
            MoralAlignment::ChaoticGood => "Хаотичный добрый",
            MoralAlignment::LawfulNeutral => "Законопослушный нейтральный",
            MoralAlignment::Neutral => "Истинно нейтральный",
            MoralAlignment::ChaoticNeutral => "Хаотичный нейтральный",
            MoralAlignment::LawfulEvil => "Законопослушный злой",
            MoralAlignment::NeutralEvil => "Нейтральный злой",
            MoralAlignment::ChaoticEvil => "Хаотичный злой",
        }
    }
}

/// Речевой профиль персонажа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechProfile {
    /// Манера речи
    pub speech_style: SpeechStyle,

    /// Сленг и жаргон
    pub slang: Vec<String>,

    /// Любимые фразы
    pub catchphrases: Vec<String>,

    /// Акцент
    pub accent: Option<String>,

    /// Темп речи
    pub speech_pace: SpeechPace,

    /// Громкость голоса
    pub voice_volume: VoiceVolume,

    /// Особенности произношения
    pub pronunciation_quirks: Vec<String>,
}

impl Default for SpeechProfile {
    fn default() -> Self {
        Self {
            speech_style: SpeechStyle::Neutral,
            slang: vec![],
            catchphrases: vec![],
            accent: None,
            speech_pace: SpeechPace::Normal,
            voice_volume: VoiceVolume::Normal,
            pronunciation_quirks: vec![],
        }
    }
}

/// Стиль речи.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechStyle {
    /// Формальный
    Formal,

    /// Неформальный
    Informal,

    /// Поэтичный
    Poetic,

    /// Грубый
    Rough,

    /// Интеллигентный
    Intellectual,

    /// Простонародный
    Colloquial,

    /// Нейтральный
    Neutral,
}

impl SpeechStyle {
    pub fn name_ru(&self) -> &'static str {
        match self {
            SpeechStyle::Formal => "Формальный",
            SpeechStyle::Informal => "Неформальный",
            SpeechStyle::Poetic => "Поэтичный",
            SpeechStyle::Rough => "Грубый",
            SpeechStyle::Intellectual => "Интеллигентный",
            SpeechStyle::Colloquial => "Простонародный",
            SpeechStyle::Neutral => "Нейтральный",
        }
    }
}

/// Темп речи.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechPace {
    /// Очень медленный
    VerySlow,

    /// Медленный
    Slow,

    /// Нормальный
    Normal,

    /// Быстрый
    Fast,

    /// Очень быстрый
    VeryFast,
}

/// Громкость голоса.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceVolume {
    /// Шёпот
    Whisper,

    /// Тихий
    Quiet,

    /// Нормальный
    Normal,

    /// Громкий
    Loud,

    /// Очень громкий
    VeryLoud,
}

/// Внешность персонажа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceProfile {
    /// Возраст
    pub age: u32,

    /// Рост (см)
    pub height: u32,

    /// Телосложение
    pub build: BodyBuild,

    /// Цвет волос
    pub hair_color: String,

    /// Цвет глаз
    pub eye_color: String,

    /// Отличительные черты
    pub distinguishing_features: Vec<String>,

    /// Стиль одежды
    pub clothing_style: String,
}

impl Default for AppearanceProfile {
    fn default() -> Self {
        Self {
            age: 30,
            height: 170,
            build: BodyBuild::Average,
            hair_color: "Тёмные".to_string(),
            eye_color: "Карие".to_string(),
            distinguishing_features: vec![],
            clothing_style: "Повседневный".to_string(),
        }
    }
}

/// Телосложение.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyBuild {
    /// Худощавое
    Slim,

    /// Среднее
    Average,

    /// Атлетическое
    Athletic,

    /// Крупное
    Large,

    /// Мускулистое
    Muscular,
}

impl BodyBuild {
    pub fn name_ru(&self) -> &'static str {
        match self {
            BodyBuild::Slim => "Худощавое",
            BodyBuild::Average => "Среднее",
            BodyBuild::Athletic => "Атлетическое",
            BodyBuild::Large => "Крупное",
            BodyBuild::Muscular => "Мускулистое",
        }
    }
}

/// Навык персонажа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: SkillLevel,
    pub description: String,
}

/// Уровень навыка.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SkillLevel {
    /// Новичок
    Novice,

    /// Ученик
    Apprentice,

    /// Компетентный
    Competent,

    /// Эксперт
    Expert,

    /// Мастер
    Master,
}

impl SkillLevel {
    pub fn name_ru(&self) -> &'static str {
        match self {
            SkillLevel::Novice => "Новичок",
            SkillLevel::Apprentice => "Ученик",
            SkillLevel::Competent => "Компетентный",
            SkillLevel::Expert => "Эксперт",
            SkillLevel::Master => "Мастер",
        }
    }
}

/// Мотивация персонажа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motivation {
    /// Основная мотивация
    pub primary: String,

    /// Второстепенные мотивации
    pub secondary: Vec<String>,

    /// Что движет персонажем
    pub driving_force: DrivingForce,
}

impl Default for Motivation {
    fn default() -> Self {
        Self {
            primary: "Неизвестно".to_string(),
            secondary: vec![],
            driving_force: DrivingForce::Survival,
        }
    }
}

/// Движущая сила персонажа.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrivingForce {
    /// Выживание
    Survival,

    /// Любовь
    Love,

    /// Месть
    Revenge,

    /// Власть
    Power,

    /// Знания
    Knowledge,

    /// Справедливость
    Justice,

    /// Богатство
    Wealth,

    /// Слава
    Fame,

    /// Искупление
    Redemption,
}

impl DrivingForce {
    pub fn name_ru(&self) -> &'static str {
        match self {
            DrivingForce::Survival => "Выживание",
            DrivingForce::Love => "Любовь",
            DrivingForce::Revenge => "Месть",
            DrivingForce::Power => "Власть",
            DrivingForce::Knowledge => "Знания",
            DrivingForce::Justice => "Справедливость",
            DrivingForce::Wealth => "Богатство",
            DrivingForce::Fame => "Слава",
            DrivingForce::Redemption => "Искупление",
        }
    }
}

/// Тип персонажа (главный/NPC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterType {
    /// Главный персонаж
    MainCharacter,

    /// Важный второстепенный персонаж
    ImportantNPC,

    /// Обычный NPC
    RegularNPC,

    /// Эпизодический персонаж
    MinorNPC,
}

impl CharacterType {
    pub fn name_ru(&self) -> &'static str {
        match self {
            CharacterType::MainCharacter => "Главный персонаж",
            CharacterType::ImportantNPC => "Важный NPC",
            CharacterType::RegularNPC => "Обычный NPC",
            CharacterType::MinorNPC => "Эпизодический персонаж",
        }
    }
}
