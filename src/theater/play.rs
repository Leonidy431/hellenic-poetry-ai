/// Интерактивная театральная постановка.

use crate::{Result, Error};
use crate::ai::AIClient;
use crate::orthodox::HolyFathersModule;
use crate::theater::{
    Genre, Mood, Scene, SceneType, Character, VotingSystem,
    VoteOption, PlotGenerator, PlotBranch,
};
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug};
use serde::{Deserialize, Serialize};

/// Интерактивная театральная постановка.
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::theater::{InteractivePlay, Genre};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut play = InteractivePlay::new()
///         .title("Одиссея")
///         .genre(Genre::Mythological)
///         .await?;
///
///     play.start().await?;
///     play.vote("option_a", 100).await?;
///     play.next_scene().await?;
///
///     Ok(())
/// }
/// ```
pub struct InteractivePlay {
    /// Метаданные постановки
    metadata: PlayMetadata,

    /// AI клиент для генерации сюжета
    ai_client: Arc<AIClient>,

    /// Православный модуль (цензор)
    orthodox_module: Arc<HolyFathersModule>,

    /// Генератор сюжета
    plot_generator: PlotGenerator,

    /// Система голосования
    voting_system: Arc<RwLock<VotingSystem>>,

    /// Текущее состояние
    state: Arc<RwLock<PlayState>>,
}

impl InteractivePlay {
    /// Создать строитель постановки.
    pub fn new() -> PlayBuilder {
        PlayBuilder::new()
    }

    /// Запустить постановку.
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting interactive play: {}", self.metadata.title);

        let mut state = self.state.write();
        state.status = PlayStatus::Running;
        state.current_scene_index = 0;
        drop(state);

        // Генерация первой сцены
        let first_scene = self.generate_opening_scene().await?;

        let mut state = self.state.write();
        state.scenes.push(first_scene.clone());
        drop(state);

        // Проверка через православный модуль
        let check_result = self.orthodox_module
            .check_plot_orthodoxy(&first_scene.content)
            .await?;

        if !check_result.is_approved {
            info!("Scene needs adjustment: {:?}", check_result.issues);
            // Корректировка сцены
            let adjusted_scene = self.adjust_scene_content(
                &first_scene,
                &check_result.recommendations
            ).await?;

            let mut state = self.state.write();
            state.scenes[0] = adjusted_scene;
        }

        info!("Play started successfully");
        Ok(())
    }

    /// Голосование за вариант развития сюжета.
    ///
    /// # Аргументы
    ///
    /// * `option_id` - ID варианта
    /// * `votes` - Количество голосов
    pub async fn vote(&mut self, option_id: &str, votes: u32) -> Result<()> {
        debug!("Voting: {} - {} votes", option_id, votes);

        let mut voting = self.voting_system.write();
        voting.add_votes(option_id, votes)?;

        Ok(())
    }

    /// Перейти к следующей сцене на основе голосования.
    pub async fn next_scene(&mut self) -> Result<Scene> {
        info!("Generating next scene based on votes");

        // Получение результатов голосования
        let voting = self.voting_system.read();
        let winner = voting.get_winner()?;
        drop(voting);

        // Генерация следующей сцены
        let next_scene = self.generate_next_scene(&winner).await?;

        // Проверка через православный модуль
        let check_result = self.orthodox_module
            .check_plot_orthodoxy(&next_scene.content)
            .await?;

        let final_scene = if !check_result.is_approved {
            info!("Adjusting scene based on orthodox check");
            self.adjust_scene_content(&next_scene, &check_result.recommendations).await?
        } else {
            next_scene
        };

        // Сохранение сцены
        let mut state = self.state.write();
        state.scenes.push(final_scene.clone());
        state.current_scene_index += 1;

        // Сброс голосования для следующей сцены
        drop(state);
        let mut voting = self.voting_system.write();
        voting.reset();

        Ok(final_scene)
    }

    /// Получить текущую сцену.
    pub fn get_current_scene(&self) -> Option<Scene> {
        let state = self.state.read();
        state.scenes.get(state.current_scene_index).cloned()
    }

    /// Получить все сцены.
    pub fn get_all_scenes(&self) -> Vec<Scene> {
        let state = self.state.read();
        state.scenes.clone()
    }

    /// Получить статус постановки.
    pub fn get_status(&self) -> PlayStatus {
        let state = self.state.read();
        state.status
    }

    /// Завершить постановку.
    pub async fn finish(&mut self) -> Result<()> {
        info!("Finishing play: {}", self.metadata.title);

        let mut state = self.state.write();
        state.status = PlayStatus::Finished;

        Ok(())
    }

    /// Генерация открывающей сцены.
    async fn generate_opening_scene(&self) -> Result<Scene> {
        let prompt = format!(
            "Создай открывающую сцену для интерактивной театральной постановки:\n\
             Название: {}\n\
             Жанр: {}\n\
             Описание: {}\n\
             \n\
             Сцена должна:\n\
             1. Представить главных героев\n\
             2. Задать атмосферу\n\
             3. Создать интригу\n\
             4. Предложить 2-3 варианта развития сюжета",
            self.metadata.title,
            self.metadata.genre.name_ru(),
            self.metadata.description
        );

        // TODO: Реальный вызов AI
        // let content = self.ai_client.generate(&prompt).await?;

        // Временная заглушка
        let content = format!(
            "Открывающая сцена постановки '{}'.\n\
             Жанр: {}.\n\
             \n\
             Действие начинается...",
            self.metadata.title,
            self.metadata.genre.name_ru()
        );

        Ok(Scene {
            id: uuid::Uuid::new_v4().to_string(),
            scene_number: 1,
            scene_type: SceneType::Opening,
            title: "Пролог".to_string(),
            content,
            characters: vec![],
            mood: Mood::Mysterious,
            duration_minutes: 10,
        })
    }

    /// Генерация следующей сцены на основе выбора.
    async fn generate_next_scene(&self, choice: &VoteOption) -> Result<Scene> {
        let state = self.state.read();
        let scene_number = state.scenes.len() + 1;
        drop(state);

        let prompt = format!(
            "Создай следующую сцену (сцена {}) на основе выбора зрителей:\n\
             Выбор: {}\n\
             Описание: {}",
            scene_number,
            choice.title,
            choice.description
        );

        // TODO: Реальный вызов AI
        let content = format!(
            "Сцена {} развивается на основе выбора '{}'...",
            scene_number,
            choice.title
        );

        Ok(Scene {
            id: uuid::Uuid::new_v4().to_string(),
            scene_number: scene_number as u32,
            scene_type: SceneType::Development,
            title: choice.title.clone(),
            content,
            characters: vec![],
            mood: Mood::Dramatic,
            duration_minutes: 15,
        })
    }

    /// Корректировка содержимого сцены на основе рекомендаций.
    async fn adjust_scene_content(
        &self,
        scene: &Scene,
        recommendations: &[String],
    ) -> Result<Scene> {
        let prompt = format!(
            "Скорректируй сцену с учётом рекомендаций:\n\
             Оригинальная сцена: {}\n\
             Рекомендации: {:?}",
            scene.content,
            recommendations
        );

        // TODO: Реальный вызов AI для корректировки

        let mut adjusted = scene.clone();
        adjusted.content = format!(
            "{}\n\n[Скорректировано с учётом православных ценностей]",
            scene.content
        );

        Ok(adjusted)
    }
}

/// Метаданные постановки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayMetadata {
    pub id: String,
    pub title: String,
    pub description: String,
    pub genre: Genre,
    pub author: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Состояние постановки.
#[derive(Debug, Clone)]
pub struct PlayState {
    pub status: PlayStatus,
    pub scenes: Vec<Scene>,
    pub current_scene_index: usize,
    pub characters: Vec<Character>,
}

/// Статус постановки.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayStatus {
    /// Не начата
    NotStarted,

    /// Идёт
    Running,

    /// На паузе
    Paused,

    /// Завершена
    Finished,
}

/// Строитель постановки.
pub struct PlayBuilder {
    title: Option<String>,
    description: Option<String>,
    genre: Option<Genre>,
    author: Option<String>,
}

impl PlayBuilder {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            genre: None,
            author: None,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn genre(mut self, genre: Genre) -> Self {
        self.genre = Some(genre);
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    pub async fn await(self) -> Result<InteractivePlay> {
        let title = self.title.ok_or_else(|| {
            Error::InvalidInput("Title not specified".to_string())
        })?;

        let genre = self.genre.unwrap_or(Genre::Drama);
        let description = self.description.unwrap_or_else(|| {
            format!("Интерактивная постановка в жанре {}", genre.name_ru())
        });
        let author = self.author.unwrap_or_else(|| "Academia Lux".to_string());

        let ai_client = Arc::new(AIClient::new().await?);
        let orthodox_module = Arc::new(HolyFathersModule::new().await?);
        let plot_generator = PlotGenerator::new(ai_client.clone());
        let voting_system = Arc::new(RwLock::new(VotingSystem::new()));

        let metadata = PlayMetadata {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description,
            genre,
            author,
            created_at: chrono::Utc::now(),
        };

        let state = Arc::new(RwLock::new(PlayState {
            status: PlayStatus::NotStarted,
            scenes: vec![],
            current_scene_index: 0,
            characters: vec![],
        }));

        Ok(InteractivePlay {
            metadata,
            ai_client,
            orthodox_module,
            plot_generator,
            voting_system,
            state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_play_creation() {
        let play = InteractivePlay::new()
            .title("Тестовая постановка")
            .genre(Genre::Comedy)
            .await;

        assert!(play.is_ok());
    }
}
