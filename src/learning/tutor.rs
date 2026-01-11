/// Основной класс для обучения языкам.
///
/// Управляет процессом обучения, создаёт уроки, отслеживает прогресс.

use crate::{Result, Error};
use crate::ai::AIClient;
use crate::learning::{
    Language, LanguageLevel, LessonType, Lesson, LessonBuilder,
    VocabularyDatabase, Exercise, ExerciseType,
};
use std::sync::Arc;
use tracing::{info, debug};
use serde::{Deserialize, Serialize};

/// Репетитор по языкам.
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
///         .topic("приветствия")
///         .await?;
///
///     lesson.teach().await?;
///     Ok(())
/// }
/// ```
pub struct LanguageTutor {
    /// AI клиент для генерации контента
    ai_client: Arc<AIClient>,

    /// База словарного запаса
    vocabulary: VocabularyDatabase,

    /// Прогресс студента
    student_progress: StudentProgress,
}

impl LanguageTutor {
    /// Создать нового репетитора.
    pub async fn new() -> Result<Self> {
        info!("Initializing Language Tutor");

        let ai_client = Arc::new(AIClient::new().await?);
        let vocabulary = VocabularyDatabase::new().await?;
        let student_progress = StudentProgress::new();

        info!("Language Tutor initialized");

        Ok(Self {
            ai_client,
            vocabulary,
            student_progress,
        })
    }

    /// Создать новый урок.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::learning::{LanguageTutor, Language, LanguageLevel};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let tutor = LanguageTutor::new().await?;
    /// let lesson = tutor
    ///     .create_lesson()
    ///     .language(Language::Latin)
    ///     .level(LanguageLevel::A2)
    ///     .topic("глаголы")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_lesson(&self) -> LessonBuilder {
        LessonBuilder::new(self.ai_client.clone(), self.vocabulary.clone())
    }

    /// Получить рекомендованный урок на основе прогресса студента.
    ///
    /// # Аргументы
    ///
    /// * `language` - Язык обучения
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::learning::{LanguageTutor, Language};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let tutor = LanguageTutor::new().await?;
    /// let lesson = tutor.get_recommended_lesson(Language::AncientGreek).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_recommended_lesson(&self, language: Language) -> Result<Lesson> {
        debug!("Getting recommended lesson for {:?}", language);

        let progress = self.student_progress.get_language_progress(language);

        let level = progress.current_level;
        let weak_areas = progress.weak_areas();

        let topic = if !weak_areas.is_empty() {
            weak_areas[0].clone()
        } else {
            "общий обзор".to_string()
        };

        self.create_lesson()
            .language(language)
            .level(level)
            .topic(&topic)
            .await
    }

    /// Оценить упражнение студента.
    ///
    /// # Аргументы
    ///
    /// * `exercise` - Упражнение
    /// * `answer` - Ответ студента
    ///
    /// # Возвращает
    ///
    /// Результат проверки с оценкой и объяснением
    pub async fn evaluate_exercise(
        &mut self,
        exercise: &Exercise,
        answer: &str,
    ) -> Result<ExerciseEvaluation> {
        debug!("Evaluating exercise: {}", exercise.question);

        let is_correct = exercise.check_answer(answer);

        let feedback = if is_correct {
            "Отлично! Правильный ответ.".to_string()
        } else {
            format!(
                "Неправильно. Правильный ответ: {}. Попробуйте ещё раз!",
                exercise.correct_answer
            )
        };

        // Обновление прогресса
        self.student_progress.record_exercise_result(
            exercise.language,
            is_correct,
        );

        Ok(ExerciseEvaluation {
            is_correct,
            feedback,
            correct_answer: exercise.correct_answer.clone(),
            explanation: exercise.explanation.clone(),
        })
    }

    /// Получить прогресс студента по языку.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::learning::{LanguageTutor, Language};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let tutor = LanguageTutor::new().await?;
    /// let progress = tutor.get_progress(Language::ChurchSlavonic);
    /// println!("Уровень: {:?}", progress.current_level);
    /// println!("Выполнено уроков: {}", progress.lessons_completed);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_progress(&self, language: Language) -> LanguageProgress {
        self.student_progress.get_language_progress(language)
    }

    /// Получить статистику по всем языкам.
    pub fn get_all_progress(&self) -> Vec<(Language, LanguageProgress)> {
        self.student_progress.get_all_progress()
    }

    /// Создать персонализированный план обучения.
    ///
    /// # Аргументы
    ///
    /// * `language` - Язык обучения
    /// * `target_level` - Целевой уровень
    /// * `weeks` - Количество недель на обучение
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::learning::{LanguageTutor, Language, LanguageLevel};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let tutor = LanguageTutor::new().await?;
    /// let plan = tutor.create_learning_plan(
    ///     Language::Latin,
    ///     LanguageLevel::B1,
    ///     12
    /// ).await?;
    ///
    /// for week in plan.weeks {
    ///     println!("Неделя {}: {}", week.number, week.focus);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_learning_plan(
        &self,
        language: Language,
        target_level: LanguageLevel,
        weeks: u32,
    ) -> Result<LearningPlan> {
        info!(
            "Creating learning plan: {:?} -> {:?} in {} weeks",
            language, target_level, weeks
        );

        let current_progress = self.student_progress.get_language_progress(language);
        let current_level = current_progress.current_level;

        let mut plan_weeks = Vec::new();

        for week_num in 1..=weeks {
            let focus = match week_num {
                1..=4 => "Алфавит и базовая лексика",
                5..=8 => "Грамматика и простые предложения",
                9..=12 => "Чтение текстов и перевод",
                _ => "Продвинутая практика",
            };

            plan_weeks.push(WeekPlan {
                number: week_num,
                focus: focus.to_string(),
                lessons_per_week: 3,
                topics: vec![
                    format!("{} - часть {}", focus, week_num),
                ],
            });
        }

        Ok(LearningPlan {
            language,
            current_level,
            target_level,
            total_weeks: weeks,
            weeks: plan_weeks,
        })
    }

    /// Получить словарь по теме.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::learning::{LanguageTutor, Language};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let tutor = LanguageTutor::new().await?;
    /// let words = tutor.get_vocabulary(Language::AncientGreek, "еда").await?;
    ///
    /// for word in words {
    ///     println!("{} - {}", word.word, word.translation);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_vocabulary(
        &self,
        language: Language,
        topic: &str,
    ) -> Result<Vec<crate::learning::VocabularyEntry>> {
        self.vocabulary.get_by_topic(language, topic).await
    }
}

/// Прогресс студента.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentProgress {
    /// Прогресс по каждому языку
    languages: std::collections::HashMap<Language, LanguageProgress>,
}

impl StudentProgress {
    pub fn new() -> Self {
        Self {
            languages: std::collections::HashMap::new(),
        }
    }

    pub fn get_language_progress(&self, language: Language) -> LanguageProgress {
        self.languages
            .get(&language)
            .cloned()
            .unwrap_or_else(|| LanguageProgress::new(language))
    }

    pub fn record_exercise_result(&mut self, language: Language, is_correct: bool) {
        let progress = self.languages
            .entry(language)
            .or_insert_with(|| LanguageProgress::new(language));

        progress.exercises_completed += 1;
        if is_correct {
            progress.exercises_correct += 1;
        }
    }

    pub fn get_all_progress(&self) -> Vec<(Language, LanguageProgress)> {
        self.languages
            .iter()
            .map(|(lang, progress)| (*lang, progress.clone()))
            .collect()
    }
}

/// Прогресс по конкретному языку.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageProgress {
    pub language: Language,
    pub current_level: LanguageLevel,
    pub lessons_completed: u32,
    pub exercises_completed: u32,
    pub exercises_correct: u32,
    pub vocabulary_learned: u32,
}

impl LanguageProgress {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            current_level: LanguageLevel::A1,
            lessons_completed: 0,
            exercises_completed: 0,
            exercises_correct: 0,
            vocabulary_learned: 0,
        }
    }

    /// Получить процент правильных ответов.
    pub fn accuracy(&self) -> f32 {
        if self.exercises_completed == 0 {
            return 0.0;
        }
        (self.exercises_correct as f32 / self.exercises_completed as f32) * 100.0
    }

    /// Получить слабые места (темы для повторения).
    pub fn weak_areas(&self) -> Vec<String> {
        // TODO: Реализовать анализ слабых мест
        vec![]
    }
}

/// Результат оценки упражнения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseEvaluation {
    pub is_correct: bool,
    pub feedback: String,
    pub correct_answer: String,
    pub explanation: Option<String>,
}

/// План обучения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPlan {
    pub language: Language,
    pub current_level: LanguageLevel,
    pub target_level: LanguageLevel,
    pub total_weeks: u32,
    pub weeks: Vec<WeekPlan>,
}

/// План на неделю.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekPlan {
    pub number: u32,
    pub focus: String,
    pub lessons_per_week: u32,
    pub topics: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_language_tutor_creation() {
        let tutor = LanguageTutor::new().await;
        assert!(tutor.is_ok());
    }

    #[test]
    fn test_student_progress() {
        let mut progress = StudentProgress::new();
        progress.record_exercise_result(Language::AncientGreek, true);
        progress.record_exercise_result(Language::AncientGreek, false);

        let lang_progress = progress.get_language_progress(Language::AncientGreek);
        assert_eq!(lang_progress.exercises_completed, 2);
        assert_eq!(lang_progress.exercises_correct, 1);
        assert_eq!(lang_progress.accuracy(), 50.0);
    }
}
