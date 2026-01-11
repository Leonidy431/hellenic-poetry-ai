/// База словарного запаса.

use crate::{Result, Error};
use crate::learning::Language;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

/// Запись в словаре.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyEntry {
    pub id: String,
    pub language: Language,
    pub word: String,
    pub translation: String,
    pub pronunciation: Option<String>,
    pub part_of_speech: PartOfSpeech,
    pub examples: Vec<String>,
    pub topic: String,
    pub difficulty: u8,
}

impl VocabularyEntry {
    pub fn new(
        language: Language,
        word: &str,
        translation: &str,
        topic: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            language,
            word: word.to_string(),
            translation: translation.to_string(),
            pronunciation: None,
            part_of_speech: PartOfSpeech::Noun,
            examples: vec![],
            topic: topic.to_string(),
            difficulty: 1,
        }
    }

    pub fn with_pronunciation(mut self, pronunciation: &str) -> Self {
        self.pronunciation = Some(pronunciation.to_string());
        self
    }

    pub fn with_part_of_speech(mut self, pos: PartOfSpeech) -> Self {
        self.part_of_speech = pos;
        self
    }

    pub fn with_examples(mut self, examples: Vec<String>) -> Self {
        self.examples = examples;
        self
    }
}

/// Часть речи.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
}

impl PartOfSpeech {
    pub fn name_ru(&self) -> &'static str {
        match self {
            PartOfSpeech::Noun => "Существительное",
            PartOfSpeech::Verb => "Глагол",
            PartOfSpeech::Adjective => "Прилагательное",
            PartOfSpeech::Adverb => "Наречие",
            PartOfSpeech::Pronoun => "Местоимение",
            PartOfSpeech::Preposition => "Предлог",
            PartOfSpeech::Conjunction => "Союз",
            PartOfSpeech::Interjection => "Междометие",
        }
    }
}

/// База данных словарного запаса.
#[derive(Debug, Clone)]
pub struct VocabularyDatabase {
    entries: HashMap<Language, Vec<VocabularyEntry>>,
}

impl VocabularyDatabase {
    /// Создать новую базу словаря.
    pub async fn new() -> Result<Self> {
        let mut db = Self {
            entries: HashMap::new(),
        };

        // Загрузка базовых словарей
        db.load_default_vocabulary();

        Ok(db)
    }

    /// Получить слова по теме.
    pub async fn get_by_topic(
        &self,
        language: Language,
        topic: &str,
    ) -> Result<Vec<VocabularyEntry>> {
        debug!("Getting vocabulary for {:?} - {}", language, topic);

        let entries = self.entries
            .get(&language)
            .ok_or_else(|| Error::NotFound(format!("No vocabulary for {:?}", language)))?;

        let filtered: Vec<VocabularyEntry> = entries
            .iter()
            .filter(|e| e.topic.to_lowercase().contains(&topic.to_lowercase()))
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Получить случайное слово.
    pub fn get_random(&self, language: Language) -> Option<VocabularyEntry> {
        use rand::seq::SliceRandom;

        self.entries
            .get(&language)?
            .choose(&mut rand::thread_rng())
            .cloned()
    }

    /// Добавить слово в словарь.
    pub fn add_entry(&mut self, entry: VocabularyEntry) {
        self.entries
            .entry(entry.language)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    /// Загрузка базового словаря.
    fn load_default_vocabulary(&mut self) {
        // Древнегреческий
        self.add_entry(
            VocabularyEntry::new(
                Language::AncientGreek,
                "Χαῖρε",
                "Здравствуй",
                "приветствия"
            )
            .with_pronunciation("khaire")
            .with_part_of_speech(PartOfSpeech::Interjection)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::AncientGreek,
                "Εἰρήνη",
                "Мир",
                "приветствия"
            )
            .with_pronunciation("eirene")
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::AncientGreek,
                "Ἀγάπη",
                "Любовь",
                "чувства"
            )
            .with_pronunciation("agape")
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::AncientGreek,
                "Σοφία",
                "Мудрость",
                "качества"
            )
            .with_pronunciation("sophia")
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        // Латынь
        self.add_entry(
            VocabularyEntry::new(
                Language::Latin,
                "Salve",
                "Здравствуй",
                "приветствия"
            )
            .with_pronunciation("salve")
            .with_part_of_speech(PartOfSpeech::Interjection)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::Latin,
                "Pax",
                "Мир",
                "приветствия"
            )
            .with_pronunciation("paks")
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::Latin,
                "Amor",
                "Любовь",
                "чувства"
            )
            .with_pronunciation("amor")
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::Latin,
                "Veritas",
                "Истина",
                "качества"
            )
            .with_pronunciation("veritas")
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        // Церковнославянский
        self.add_entry(
            VocabularyEntry::new(
                Language::ChurchSlavonic,
                "Мі́ръ",
                "Мир",
                "приветствия"
            )
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::ChurchSlavonic,
                "Любы́",
                "Любовь",
                "чувства"
            )
            .with_part_of_speech(PartOfSpeech::Noun)
        );

        self.add_entry(
            VocabularyEntry::new(
                Language::ChurchSlavonic,
                "Бла́го",
                "Добро",
                "качества"
            )
            .with_part_of_speech(PartOfSpeech::Noun)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vocabulary_database() {
        let db = VocabularyDatabase::new().await.unwrap();

        let words = db.get_by_topic(Language::AncientGreek, "приветствия")
            .await
            .unwrap();

        assert!(!words.is_empty());
    }

    #[tokio::test]
    async fn test_get_random() {
        let db = VocabularyDatabase::new().await.unwrap();
        let word = db.get_random(Language::Latin);
        assert!(word.is_some());
    }
}
