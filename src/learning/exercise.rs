/// Упражнения для изучения языков.

use crate::learning::{Language, LanguageLevel};
use serde::{Deserialize, Serialize};

/// Упражнение.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub language: Language,
    pub exercise_type: ExerciseType,
    pub question: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub explanation: Option<String>,
    pub difficulty: LanguageLevel,
}

impl Exercise {
    /// Проверить ответ студента.
    pub fn check_answer(&self, answer: &str) -> bool {
        self.correct_answer.trim().to_lowercase() == answer.trim().to_lowercase()
    }

    /// Получить подсказку.
    pub fn get_hint(&self) -> String {
        match self.exercise_type {
            ExerciseType::Translation => {
                format!("Подсказка: Ответ начинается с '{}'",
                    self.correct_answer.chars().next().unwrap_or('?'))
            }
            ExerciseType::MultipleChoice => {
                "Подсказка: Выберите один из предложенных вариантов".to_string()
            }
            ExerciseType::FillInTheBlank => {
                "Подсказка: Заполните пропуск подходящим словом".to_string()
            }
            ExerciseType::Listening => {
                "Подсказка: Прослушайте аудио ещё раз".to_string()
            }
            ExerciseType::Speaking => {
                "Подсказка: Произнесите фразу вслух".to_string()
            }
            ExerciseType::Writing => {
                "Подсказка: Напишите текст на изучаемом языке".to_string()
            }
        }
    }
}

/// Тип упражнения.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseType {
    /// Перевод
    Translation,

    /// Множественный выбор
    MultipleChoice,

    /// Заполнить пропуски
    FillInTheBlank,

    /// Аудирование
    Listening,

    /// Говорение
    Speaking,

    /// Письмо
    Writing,
}

impl ExerciseType {
    /// Получить название типа упражнения.
    pub fn name(&self) -> &'static str {
        match self {
            ExerciseType::Translation => "Перевод",
            ExerciseType::MultipleChoice => "Множественный выбор",
            ExerciseType::FillInTheBlank => "Заполнить пропуски",
            ExerciseType::Listening => "Аудирование",
            ExerciseType::Speaking => "Говорение",
            ExerciseType::Writing => "Письмо",
        }
    }
}

/// Результат выполнения упражнения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseResult {
    pub exercise_id: String,
    pub student_answer: String,
    pub is_correct: bool,
    pub time_spent_seconds: u32,
    pub attempts: u32,
}

impl ExerciseResult {
    pub fn new(exercise_id: String, student_answer: String, is_correct: bool) -> Self {
        Self {
            exercise_id,
            student_answer,
            is_correct,
            time_spent_seconds: 0,
            attempts: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_answer() {
        let exercise = Exercise {
            id: "test".to_string(),
            language: Language::AncientGreek,
            exercise_type: ExerciseType::Translation,
            question: "Переведите: Χαῖρε".to_string(),
            correct_answer: "Здравствуй".to_string(),
            options: vec![],
            explanation: None,
            difficulty: LanguageLevel::A1,
        };

        assert!(exercise.check_answer("Здравствуй"));
        assert!(exercise.check_answer("здравствуй"));
        assert!(exercise.check_answer(" Здравствуй "));
        assert!(!exercise.check_answer("Привет"));
    }
}
