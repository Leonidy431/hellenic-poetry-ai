/// Грамматические правила и объяснения.

use crate::learning::Language;
use serde::{Deserialize, Serialize};

/// Грамматическое правило.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarRule {
    pub id: String,
    pub language: Language,
    pub category: GrammarCategory,
    pub title: String,
    pub explanation: String,
    pub examples: Vec<GrammarExample>,
    pub exceptions: Vec<String>,
}

impl GrammarRule {
    pub fn new(
        language: Language,
        category: GrammarCategory,
        title: &str,
        explanation: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            language,
            category,
            title: title.to_string(),
            explanation: explanation.to_string(),
            examples: vec![],
            exceptions: vec![],
        }
    }

    pub fn with_examples(mut self, examples: Vec<GrammarExample>) -> Self {
        self.examples = examples;
        self
    }

    pub fn with_exceptions(mut self, exceptions: Vec<String>) -> Self {
        self.exceptions = exceptions;
        self
    }
}

/// Категория грамматики.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrammarCategory {
    /// Склонение существительных
    NounDeclension,

    /// Спряжение глаголов
    VerbConjugation,

    /// Падежи
    Cases,

    /// Времена
    Tenses,

    /// Залоги
    Voices,

    /// Наклонения
    Moods,

    /// Артикли
    Articles,

    /// Порядок слов
    WordOrder,

    /// Синтаксис
    Syntax,
}

impl GrammarCategory {
    pub fn name_ru(&self) -> &'static str {
        match self {
            GrammarCategory::NounDeclension => "Склонение существительных",
            GrammarCategory::VerbConjugation => "Спряжение глаголов",
            GrammarCategory::Cases => "Падежи",
            GrammarCategory::Tenses => "Времена",
            GrammarCategory::Voices => "Залоги",
            GrammarCategory::Moods => "Наклонения",
            GrammarCategory::Articles => "Артикли",
            GrammarCategory::WordOrder => "Порядок слов",
            GrammarCategory::Syntax => "Синтаксис",
        }
    }
}

/// Пример грамматического правила.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarExample {
    pub original: String,
    pub translation: String,
    pub explanation: Option<String>,
}

impl GrammarExample {
    pub fn new(original: &str, translation: &str) -> Self {
        Self {
            original: original.to_string(),
            translation: translation.to_string(),
            explanation: None,
        }
    }

    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }
}

/// Объяснение грамматики.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarExplanation {
    pub rule: GrammarRule,
    pub detailed_explanation: String,
    pub practice_exercises: Vec<String>,
}

impl GrammarExplanation {
    pub fn new(rule: GrammarRule, detailed_explanation: String) -> Self {
        Self {
            rule,
            detailed_explanation,
            practice_exercises: vec![],
        }
    }

    pub fn with_practice(mut self, exercises: Vec<String>) -> Self {
        self.practice_exercises = exercises;
        self
    }
}

/// База грамматических правил.
pub struct GrammarDatabase {
    rules: Vec<GrammarRule>,
}

impl GrammarDatabase {
    pub fn new() -> Self {
        let mut db = Self { rules: vec![] };
        db.load_default_rules();
        db
    }

    /// Получить правила по категории.
    pub fn get_by_category(
        &self,
        language: Language,
        category: GrammarCategory,
    ) -> Vec<&GrammarRule> {
        self.rules
            .iter()
            .filter(|r| r.language == language && r.category == category)
            .collect()
    }

    /// Добавить правило.
    pub fn add_rule(&mut self, rule: GrammarRule) {
        self.rules.push(rule);
    }

    /// Загрузка базовых правил.
    fn load_default_rules(&mut self) {
        // Древнегреческий - склонение существительных
        self.add_rule(
            GrammarRule::new(
                Language::AncientGreek,
                GrammarCategory::NounDeclension,
                "Первое склонение",
                "Существительные первого склонения оканчиваются на -α или -η"
            )
            .with_examples(vec![
                GrammarExample::new("ἡ χώρα", "страна")
                    .with_explanation("Именительный падеж единственного числа"),
                GrammarExample::new("τῆς χώρας", "страны")
                    .with_explanation("Родительный падеж единственного числа"),
            ])
        );

        // Латынь - спряжение глаголов
        self.add_rule(
            GrammarRule::new(
                Language::Latin,
                GrammarCategory::VerbConjugation,
                "Первое спряжение",
                "Глаголы первого спряжения имеют основу на -ā"
            )
            .with_examples(vec![
                GrammarExample::new("amō", "я люблю"),
                GrammarExample::new("amās", "ты любишь"),
                GrammarExample::new("amat", "он/она любит"),
            ])
        );

        // Церковнославянский - падежи
        self.add_rule(
            GrammarRule::new(
                Language::ChurchSlavonic,
                GrammarCategory::Cases,
                "Звательный падеж",
                "Звательный падеж используется для обращения"
            )
            .with_examples(vec![
                GrammarExample::new("Го́споди", "Господи")
                    .with_explanation("Обращение к Богу"),
                GrammarExample::new("Бра́те", "Брате")
                    .with_explanation("Обращение к брату"),
            ])
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grammar_database() {
        let db = GrammarDatabase::new();
        let rules = db.get_by_category(
            Language::AncientGreek,
            GrammarCategory::NounDeclension
        );
        assert!(!rules.is_empty());
    }
}
