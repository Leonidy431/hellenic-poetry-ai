/// Урок по языку.

use crate::{Result, Error};
use crate::ai::AIClient;
use crate::learning::{
    Language, LanguageLevel, LessonType, Exercise, ExerciseType,
    VocabularyDatabase, VocabularyEntry,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// Урок по языку.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub language: Language,
    pub level: LanguageLevel,
    pub lesson_type: LessonType,
    pub title: String,
    pub topic: String,
    pub content: LessonContent,
    pub exercises: Vec<Exercise>,
    pub vocabulary: Vec<VocabularyEntry>,
}

impl Lesson {
    /// Провести урок (вывести содержимое).
    pub async fn teach(&self) -> Result<()> {
        info!("Starting lesson: {}", self.title);

        println!("\n📚 Урок: {}", self.title);
        println!("🌍 Язык: {}", self.language.name_ru());
        println!("📊 Уровень: {:?}", self.level);
        println!("📖 Тема: {}\n", self.topic);

        // Теория
        if let Some(theory) = &self.content.theory {
            println!("📝 Теория:\n{}\n", theory);
        }

        // Примеры
        if !self.content.examples.is_empty() {
            println!("💡 Примеры:");
            for (i, example) in self.content.examples.iter().enumerate() {
                println!("  {}. {} - {}", i + 1, example.original, example.translation);
            }
            println!();
        }

        // Словарь
        if !self.vocabulary.is_empty() {
            println!("📖 Новые слова:");
            for word in &self.vocabulary {
                println!("  • {} - {}", word.word, word.translation);
            }
            println!();
        }

        // Упражнения
        if !self.exercises.is_empty() {
            println!("✏️  Упражнения: {} заданий", self.exercises.len());
        }

        info!("Lesson completed: {}", self.title);
        Ok(())
    }

    /// Получить упражнения урока.
    pub fn get_exercises(&self) -> &[Exercise] {
        &self.exercises
    }
}

/// Содержимое урока.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonContent {
    /// Теоретическая часть
    pub theory: Option<String>,

    /// Примеры
    pub examples: Vec<Example>,

    /// Грамматические правила
    pub grammar_rules: Vec<String>,

    /// Культурные заметки
    pub cultural_notes: Option<String>,
}

/// Пример в уроке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub original: String,
    pub translation: String,
    pub explanation: Option<String>,
}

/// Строитель урока.
pub struct LessonBuilder {
    ai_client: Arc<AIClient>,
    vocabulary: VocabularyDatabase,
    language: Option<Language>,
    level: Option<LanguageLevel>,
    lesson_type: Option<LessonType>,
    topic: Option<String>,
}

impl LessonBuilder {
    pub fn new(ai_client: Arc<AIClient>, vocabulary: VocabularyDatabase) -> Self {
        Self {
            ai_client,
            vocabulary,
            language: None,
            level: None,
            lesson_type: None,
            topic: None,
        }
    }

    /// Установить язык.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Установить уровень.
    pub fn level(mut self, level: LanguageLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Установить тип урока.
    pub fn lesson_type(mut self, lesson_type: LessonType) -> Self {
        self.lesson_type = Some(lesson_type);
        self
    }

    /// Установить тему.
    pub fn topic(mut self, topic: &str) -> Self {
        self.topic = Some(topic.to_string());
        self
    }

    /// Создать урок.
    pub async fn await(self) -> Result<Lesson> {
        let language = self.language.ok_or_else(|| {
            Error::InvalidInput("Language not specified".to_string())
        })?;

        let level = self.level.unwrap_or(LanguageLevel::A1);
        let lesson_type = self.lesson_type.unwrap_or(LessonType::Grammar);
        let topic = self.topic.unwrap_or_else(|| "общий обзор".to_string());

        debug!(
            "Creating lesson: {:?} {:?} - {}",
            language, level, topic
        );

        // Генерация содержимого урока через AI
        let content = self.generate_lesson_content(
            language,
            level,
            lesson_type,
            &topic,
        ).await?;

        // Получение словаря по теме
        let vocabulary = self.vocabulary
            .get_by_topic(language, &topic)
            .await
            .unwrap_or_default();

        // Генерация упражнений
        let exercises = self.generate_exercises(
            language,
            level,
            &topic,
            5,
        ).await?;

        let lesson = Lesson {
            id: uuid::Uuid::new_v4().to_string(),
            language,
            level,
            lesson_type,
            title: format!("{}: {}", language.name_ru(), topic),
            topic,
            content,
            exercises,
            vocabulary,
        };

        Ok(lesson)
    }

    /// Генерация содержимого урока через AI.
    async fn generate_lesson_content(
        &self,
        language: Language,
        level: LanguageLevel,
        lesson_type: LessonType,
        topic: &str,
    ) -> Result<LessonContent> {
        let prompt = format!(
            "Создай урок по языку {} уровня {:?} на тему '{}'. \
             Тип урока: {:?}. \
             \
             Включи: \
             1. Теоретическую часть (объяснение) \
             2. 3-5 примеров с переводом \
             3. Грамматические правила (если применимо) \
             4. Культурные заметки \
             \
             Формат ответа: JSON",
            language.name_ru(),
            level,
            topic,
            lesson_type
        );

        // TODO: Реальный вызов AI
        // let response = self.ai_client.generate(&prompt).await?;

        // Временная заглушка
        Ok(LessonContent {
            theory: Some(format!(
                "Теория по теме '{}' для уровня {:?}. \
                 В этом уроке мы изучим основы {}.",
                topic, level, topic
            )),
            examples: vec![
                Example {
                    original: "Χαῖρε".to_string(),
                    translation: "Здравствуй".to_string(),
                    explanation: Some("Приветствие в древнегреческом".to_string()),
                },
                Example {
                    original: "Εἰρήνη".to_string(),
                    translation: "Мир".to_string(),
                    explanation: Some("Пожелание мира".to_string()),
                },
            ],
            grammar_rules: vec![
                "Правило 1: Основы грамматики".to_string(),
                "Правило 2: Склонения и спряжения".to_string(),
            ],
            cultural_notes: Some(
                "Культурная заметка: В древнегреческой культуре...".to_string()
            ),
        })
    }

    /// Генерация упражнений.
    async fn generate_exercises(
        &self,
        language: Language,
        level: LanguageLevel,
        topic: &str,
        count: usize,
    ) -> Result<Vec<Exercise>> {
        let mut exercises = Vec::new();

        for i in 0..count {
            exercises.push(Exercise {
                id: uuid::Uuid::new_v4().to_string(),
                language,
                exercise_type: ExerciseType::Translation,
                question: format!("Переведите: Пример {}", i + 1),
                correct_answer: format!("Ответ {}", i + 1),
                options: vec![
                    format!("Ответ {}", i + 1),
                    format!("Неправильный ответ 1"),
                    format!("Неправильный ответ 2"),
                ],
                explanation: Some(format!("Объяснение для упражнения {}", i + 1)),
                difficulty: level,
            });
        }

        Ok(exercises)
    }
}
