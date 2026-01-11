/// Модуль создания сериалов (многосерийных постановок).
///
/// Позволяет создавать сериалы с эпизодами, сезонами и арками.

use crate::{Result, Error};
use crate::ai::AIClient;
use crate::theater::{Genre, Scene, Character, Mood};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::{info, debug};

/// Сериал (многосерийная постановка).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub id: String,
    pub title: String,
    pub description: String,
    pub genre: Genre,
    pub seasons: Vec<Season>,
    pub main_characters: Vec<Character>,
    pub status: SeriesStatus,
}

impl Series {
    /// Создать новый сериал.
    pub fn new(title: &str, description: &str, genre: Genre) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            genre,
            seasons: vec![],
            main_characters: vec![],
            status: SeriesStatus::InDevelopment,
        }
    }

    /// Добавить сезон.
    pub fn add_season(&mut self, season: Season) {
        self.seasons.push(season);
    }

    /// Получить общее количество эпизодов.
    pub fn total_episodes(&self) -> usize {
        self.seasons.iter().map(|s| s.episodes.len()).sum()
    }

    /// Получить последний эпизод.
    pub fn latest_episode(&self) -> Option<&Episode> {
        self.seasons.last()?.episodes.last()
    }
}

/// Сезон сериала.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: String,
    pub season_number: u32,
    pub title: String,
    pub description: String,
    pub episodes: Vec<Episode>,
    pub story_arc: Option<StoryArc>,
}

impl Season {
    pub fn new(season_number: u32, title: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            season_number,
            title: title.to_string(),
            description: description.to_string(),
            episodes: vec![],
            story_arc: None,
        }
    }

    pub fn with_story_arc(mut self, arc: StoryArc) -> Self {
        self.story_arc = Some(arc);
        self
    }

    pub fn add_episode(&mut self, episode: Episode) {
        self.episodes.push(episode);
    }
}

/// Эпизод сериала.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub episode_number: u32,
    pub season_number: u32,
    pub title: String,
    pub synopsis: String,
    pub scenes: Vec<Scene>,
    pub duration_minutes: u32,
    pub cliffhanger: Option<String>,
}

impl Episode {
    pub fn new(
        season_number: u32,
        episode_number: u32,
        title: &str,
        synopsis: &str,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            episode_number,
            season_number,
            title: title.to_string(),
            synopsis: synopsis.to_string(),
            scenes: vec![],
            duration_minutes: 45,
            cliffhanger: None,
        }
    }

    pub fn with_cliffhanger(mut self, cliffhanger: &str) -> Self {
        self.cliffhanger = Some(cliffhanger.to_string());
        self
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    /// Получить полный номер эпизода (S01E01).
    pub fn full_number(&self) -> String {
        format!("S{:02}E{:02}", self.season_number, self.episode_number)
    }
}

/// Сюжетная арка.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryArc {
    pub id: String,
    pub title: String,
    pub description: String,
    pub start_episode: u32,
    pub end_episode: u32,
    pub main_conflict: String,
    pub resolution: Option<String>,
}

impl StoryArc {
    pub fn new(
        title: &str,
        description: &str,
        start_episode: u32,
        end_episode: u32,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            start_episode,
            end_episode,
            main_conflict: String::new(),
            resolution: None,
        }
    }

    pub fn with_conflict(mut self, conflict: &str) -> Self {
        self.main_conflict = conflict.to_string();
        self
    }

    pub fn with_resolution(mut self, resolution: &str) -> Self {
        self.resolution = Some(resolution.to_string());
        self
    }
}

/// Статус сериала.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeriesStatus {
    /// В разработке
    InDevelopment,

    /// Идёт показ
    Airing,

    /// На паузе
    OnHiatus,

    /// Завершён
    Completed,

    /// Отменён
    Cancelled,
}

impl SeriesStatus {
    pub fn name_ru(&self) -> &'static str {
        match self {
            SeriesStatus::InDevelopment => "В разработке",
            SeriesStatus::Airing => "Идёт показ",
            SeriesStatus::OnHiatus => "На паузе",
            SeriesStatus::Completed => "Завершён",
            SeriesStatus::Cancelled => "Отменён",
        }
    }
}

/// Генератор сериалов.
pub struct SeriesGenerator {
    ai_client: Arc<AIClient>,
}

impl SeriesGenerator {
    pub fn new(ai_client: Arc<AIClient>) -> Self {
        Self { ai_client }
    }

    /// Сгенерировать сериал.
    ///
    /// # Аргументы
    ///
    /// * `title` - Название сериала
    /// * `genre` - Жанр
    /// * `seasons_count` - Количество сезонов
    /// * `episodes_per_season` - Эпизодов в сезоне
    pub async fn generate_series(
        &self,
        title: &str,
        genre: Genre,
        seasons_count: u32,
        episodes_per_season: u32,
    ) -> Result<Series> {
        info!("Generating series: {} ({} seasons)", title, seasons_count);

        let mut series = Series::new(
            title,
            &format!("Сериал в жанре {}", genre.name_ru()),
            genre,
        );

        for season_num in 1..=seasons_count {
            let season = self.generate_season(
                &series,
                season_num,
                episodes_per_season,
            ).await?;

            series.add_season(season);
        }

        series.status = SeriesStatus::InDevelopment;

        Ok(series)
    }

    /// Сгенерировать сезон.
    async fn generate_season(
        &self,
        series: &Series,
        season_number: u32,
        episodes_count: u32,
    ) -> Result<Season> {
        debug!("Generating season {}", season_number);

        let mut season = Season::new(
            season_number,
            &format!("Сезон {}", season_number),
            &format!("Описание сезона {}", season_number),
        );

        // Создание сюжетной арки для сезона
        let arc = StoryArc::new(
            &format!("Арка сезона {}", season_number),
            "Основная сюжетная линия сезона",
            1,
            episodes_count,
        )
        .with_conflict("Главный конфликт сезона");

        season = season.with_story_arc(arc);

        // Генерация эпизодов
        for ep_num in 1..=episodes_count {
            let episode = self.generate_episode(
                series,
                season_number,
                ep_num,
            ).await?;

            season.add_episode(episode);
        }

        Ok(season)
    }

    /// Сгенерировать эпизод.
    async fn generate_episode(
        &self,
        series: &Series,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Episode> {
        debug!("Generating episode S{:02}E{:02}", season_number, episode_number);

        let prompt = format!(
            "Создай эпизод для сериала '{}':\n\
             Жанр: {}\n\
             Сезон: {}, Эпизод: {}\n\
             \n\
             Включи:\n\
             1. Название эпизода\n\
             2. Краткое описание (синопсис)\n\
             3. Основные события\n\
             4. Клиффхэнгер (если это не финал сезона)",
            series.title,
            series.genre.name_ru(),
            season_number,
            episode_number
        );

        // TODO: Реальный вызов AI
        // let response = self.ai_client.generate(&prompt).await?;

        let mut episode = Episode::new(
            season_number,
            episode_number,
            &format!("Эпизод {}", episode_number),
            &format!("Синопсис эпизода {}", episode_number),
        );

        // Генерация сцен для эпизода
        for scene_num in 1..=5 {
            let scene = Scene::new(
                scene_num,
                &format!("Сцена {}", scene_num),
                &format!("Содержимое сцены {}", scene_num),
            )
            .with_mood(Mood::Dramatic);

            episode.add_scene(scene);
        }

        // Добавление клиффхэнгера для некоторых эпизодов
        if episode_number % 3 == 0 {
            episode = episode.with_cliffhanger(
                "Неожиданный поворот событий оставляет героев в опасности..."
            );
        }

        Ok(episode)
    }

    /// Сгенерировать следующий эпизод на основе предыдущих.
    pub async fn generate_next_episode(
        &self,
        series: &Series,
        previous_episodes: &[Episode],
    ) -> Result<Episode> {
        let last_episode = previous_episodes.last()
            .ok_or_else(|| Error::InvalidInput("No previous episodes".to_string()))?;

        let next_episode_num = last_episode.episode_number + 1;

        self.generate_episode(
            series,
            last_episode.season_number,
            next_episode_num,
        ).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_creation() {
        let mut series = Series::new(
            "Тестовый сериал",
            "Описание",
            Genre::Drama,
        );

        let mut season = Season::new(1, "Сезон 1", "Первый сезон");
        let episode = Episode::new(1, 1, "Пилот", "Первый эпизод");

        season.add_episode(episode);
        series.add_season(season);

        assert_eq!(series.total_episodes(), 1);
        assert_eq!(series.latest_episode().unwrap().title, "Пилот");
    }

    #[test]
    fn test_episode_full_number() {
        let episode = Episode::new(1, 5, "Тест", "Описание");
        assert_eq!(episode.full_number(), "S01E05");
    }
}
