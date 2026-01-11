/// Персонажи театральной постановки.

use serde::{Deserialize, Serialize};

/// Персонаж.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub role: CharacterRole,
    pub description: String,
    pub personality: Vec<String>,
    pub relationships: Vec<CharacterRelationship>,
    pub backstory: Option<String>,
}

impl Character {
    pub fn new(name: &str, role: CharacterRole, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            role,
            description: description.to_string(),
            personality: vec![],
            relationships: vec![],
            backstory: None,
        }
    }

    pub fn with_personality(mut self, traits: Vec<String>) -> Self {
        self.personality = traits;
        self
    }

    pub fn with_backstory(mut self, backstory: &str) -> Self {
        self.backstory = Some(backstory.to_string());
        self
    }

    pub fn add_relationship(&mut self, relationship: CharacterRelationship) {
        self.relationships.push(relationship);
    }
}

/// Роль персонажа.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterRole {
    /// Главный герой
    Protagonist,

    /// Антагонист
    Antagonist,

    /// Второстепенный персонаж
    Supporting,

    /// Наставник
    Mentor,

    /// Комический персонаж
    Comic,

    /// Злодей
    Villain,

    /// Любовный интерес
    LoveInterest,

    /// Рассказчик
    Narrator,
}

impl CharacterRole {
    pub fn name_ru(&self) -> &'static str {
        match self {
            CharacterRole::Protagonist => "Главный герой",
            CharacterRole::Antagonist => "Антагонист",
            CharacterRole::Supporting => "Второстепенный персонаж",
            CharacterRole::Mentor => "Наставник",
            CharacterRole::Comic => "Комический персонаж",
            CharacterRole::Villain => "Злодей",
            CharacterRole::LoveInterest => "Любовный интерес",
            CharacterRole::Narrator => "Рассказчик",
        }
    }
}

/// Отношения между персонажами.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelationship {
    pub character_id: String,
    pub relationship_type: RelationshipType,
    pub description: String,
}

/// Тип отношений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Друзья
    Friend,

    /// Враги
    Enemy,

    /// Семья
    Family,

    /// Романтические отношения
    Romantic,

    /// Наставник-ученик
    MentorStudent,

    /// Соперники
    Rival,

    /// Союзники
    Ally,

    /// Нейтральные
    Neutral,
}

impl RelationshipType {
    pub fn name_ru(&self) -> &'static str {
        match self {
            RelationshipType::Friend => "Друзья",
            RelationshipType::Enemy => "Враги",
            RelationshipType::Family => "Семья",
            RelationshipType::Romantic => "Романтические отношения",
            RelationshipType::MentorStudent => "Наставник-ученик",
            RelationshipType::Rival => "Соперники",
            RelationshipType::Ally => "Союзники",
            RelationshipType::Neutral => "Нейтральные",
        }
    }
}
