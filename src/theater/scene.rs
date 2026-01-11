/// Сцены театральной постановки.

use crate::theater::{Mood, Character};
use serde::{Deserialize, Serialize};

/// Сцена постановки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub scene_number: u32,
    pub scene_type: SceneType,
    pub title: String,
    pub content: String,
    pub characters: Vec<Character>,
    pub mood: Mood,
    pub duration_minutes: u32,
}

impl Scene {
    pub fn new(scene_number: u32, title: &str, content: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            scene_number,
            scene_type: SceneType::Development,
            title: title.to_string(),
            content: content.to_string(),
            characters: vec![],
            mood: Mood::Calm,
            duration_minutes: 10,
        }
    }

    pub fn with_type(mut self, scene_type: SceneType) -> Self {
        self.scene_type = scene_type;
        self
    }

    pub fn with_mood(mut self, mood: Mood) -> Self {
        self.mood = mood;
        self
    }

    pub fn with_characters(mut self, characters: Vec<Character>) -> Self {
        self.characters = characters;
        self
    }

    pub fn with_duration(mut self, minutes: u32) -> Self {
        self.duration_minutes = minutes;
        self
    }
}

/// Тип сцены.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneType {
    /// Открывающая сцена
    Opening,

    /// Развитие сюжета
    Development,

    /// Кульминация
    Climax,

    /// Развязка
    Resolution,

    /// Эпилог
    Epilogue,

    /// Интерлюдия (переходная сцена)
    Interlude,
}

impl SceneType {
    pub fn name_ru(&self) -> &'static str {
        match self {
            SceneType::Opening => "Открывающая сцена",
            SceneType::Development => "Развитие сюжета",
            SceneType::Climax => "Кульминация",
            SceneType::Resolution => "Развязка",
            SceneType::Epilogue => "Эпилог",
            SceneType::Interlude => "Интерлюдия",
        }
    }
}

/// Переход между сценами.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneTransition {
    pub from_scene_id: String,
    pub to_scene_id: String,
    pub transition_type: TransitionType,
    pub description: String,
}

/// Тип перехода.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    /// Плавный переход
    Smooth,

    /// Резкий переход
    Abrupt,

    /// Затемнение
    Fade,

    /// Флешбэк
    Flashback,

    /// Флешфорвард
    Flashforward,
}

impl TransitionType {
    pub fn name_ru(&self) -> &'static str {
        match self {
            TransitionType::Smooth => "Плавный переход",
            TransitionType::Abrupt => "Резкий переход",
            TransitionType::Fade => "Затемнение",
            TransitionType::Flashback => "Флешбэк",
            TransitionType::Flashforward => "Флешфорвард",
        }
    }
}
