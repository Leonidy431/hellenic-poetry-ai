/// Система голосования для интерактивного театра.

use crate::{Result, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Система голосования.
#[derive(Debug, Clone)]
pub struct VotingSystem {
    /// Текущие варианты для голосования
    options: Vec<VoteOption>,

    /// Результаты голосования
    votes: HashMap<String, u32>,

    /// Статус голосования
    status: VotingStatus,
}

impl VotingSystem {
    pub fn new() -> Self {
        Self {
            options: vec![],
            votes: HashMap::new(),
            status: VotingStatus::NotStarted,
        }
    }

    /// Начать новое голосование.
    pub fn start_voting(&mut self, options: Vec<VoteOption>) -> Result<()> {
        self.options = options;
        self.votes.clear();
        self.status = VotingStatus::Active;

        // Инициализация счётчиков
        for option in &self.options {
            self.votes.insert(option.id.clone(), 0);
        }

        Ok(())
    }

    /// Добавить голоса за вариант.
    pub fn add_votes(&mut self, option_id: &str, count: u32) -> Result<()> {
        if self.status != VotingStatus::Active {
            return Err(Error::InvalidInput("Voting is not active".to_string()));
        }

        let votes = self.votes.get_mut(option_id)
            .ok_or_else(|| Error::NotFound(format!("Option not found: {}", option_id)))?;

        *votes += count;

        Ok(())
    }

    /// Получить победителя голосования.
    pub fn get_winner(&self) -> Result<VoteOption> {
        if self.votes.is_empty() {
            return Err(Error::InvalidInput("No votes recorded".to_string()));
        }

        let winner_id = self.votes
            .iter()
            .max_by_key(|(_, &votes)| votes)
            .map(|(id, _)| id)
            .ok_or_else(|| Error::Unknown("Failed to determine winner".to_string()))?;

        self.options
            .iter()
            .find(|opt| &opt.id == winner_id)
            .cloned()
            .ok_or_else(|| Error::NotFound("Winner option not found".to_string()))
    }

    /// Получить результаты голосования.
    pub fn get_results(&self) -> VoteResult {
        let mut results = vec![];

        for option in &self.options {
            let votes = self.votes.get(&option.id).copied().unwrap_or(0);
            results.push((option.clone(), votes));
        }

        // Сортировка по количеству голосов
        results.sort_by(|a, b| b.1.cmp(&a.1));

        let total_votes: u32 = self.votes.values().sum();

        VoteResult {
            options: results,
            total_votes,
            status: self.status,
        }
    }

    /// Завершить голосование.
    pub fn finish_voting(&mut self) {
        self.status = VotingStatus::Finished;
    }

    /// Сбросить голосование.
    pub fn reset(&mut self) {
        self.options.clear();
        self.votes.clear();
        self.status = VotingStatus::NotStarted;
    }
}

/// Вариант для голосования.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteOption {
    pub id: String,
    pub title: String,
    pub description: String,
    pub consequences: Option<String>,
}

impl VoteOption {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            consequences: None,
        }
    }

    pub fn with_consequences(mut self, consequences: &str) -> Self {
        self.consequences = Some(consequences.to_string());
        self
    }
}

/// Результат голосования.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResult {
    pub options: Vec<(VoteOption, u32)>,
    pub total_votes: u32,
    pub status: VotingStatus,
}

impl VoteResult {
    /// Получить победителя.
    pub fn winner(&self) -> Option<&VoteOption> {
        self.options.first().map(|(opt, _)| opt)
    }

    /// Получить процент голосов за вариант.
    pub fn percentage(&self, option_id: &str) -> f32 {
        if self.total_votes == 0 {
            return 0.0;
        }

        let votes = self.options
            .iter()
            .find(|(opt, _)| opt.id == option_id)
            .map(|(_, votes)| votes)
            .copied()
            .unwrap_or(0);

        (votes as f32 / self.total_votes as f32) * 100.0
    }
}

/// Статус голосования.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VotingStatus {
    /// Не начато
    NotStarted,

    /// Активно
    Active,

    /// Завершено
    Finished,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voting_system() {
        let mut system = VotingSystem::new();

        let options = vec![
            VoteOption::new("Вариант A", "Описание A"),
            VoteOption::new("Вариант B", "Описание B"),
        ];

        system.start_voting(options.clone()).unwrap();
        system.add_votes(&options[0].id, 100).unwrap();
        system.add_votes(&options[1].id, 150).unwrap();

        let winner = system.get_winner().unwrap();
        assert_eq!(winner.title, "Вариант B");

        let results = system.get_results();
        assert_eq!(results.total_votes, 250);
    }
}
