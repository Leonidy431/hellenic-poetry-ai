/// VK Mini App для Academia Lux.
///
/// Предоставляет полноценный фронтенд через VK Mini Apps.

use crate::{Result, Error};
use serde::{Deserialize, Serialize};

/// Конфигурация VK Mini App.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniAppConfig {
    /// ID приложения
    pub app_id: i64,

    /// Секретный ключ
    pub secret_key: String,

    /// Service Token
    pub service_token: String,
}

impl MiniAppConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            app_id: std::env::var("VK_APP_ID")
                .map_err(|_| Error::ConfigError("VK_APP_ID not set".to_string()))?
                .parse()
                .map_err(|_| Error::ConfigError("Invalid VK_APP_ID".to_string()))?,
            secret_key: std::env::var("VK_APP_SECRET")
                .map_err(|_| Error::ConfigError("VK_APP_SECRET not set".to_string()))?,
            service_token: std::env::var("VK_SERVICE_TOKEN")
                .map_err(|_| Error::ConfigError("VK_SERVICE_TOKEN not set".to_string()))?,
        })
    }
}

/// VK Mini App.
pub struct VKMiniApp {
    config: MiniAppConfig,
}

impl VKMiniApp {
    pub fn new(config: MiniAppConfig) -> Self {
        Self { config }
    }

    /// Проверить подпись запроса от VK.
    pub fn verify_launch_params(&self, params: &str) -> Result<bool> {
        // TODO: Реализовать проверку подписи
        // https://dev.vk.com/mini-apps/development/launch-params

        Ok(true)
    }

    /// Получить информацию о пользователе из launch params.
    pub fn parse_user_info(&self, params: &str) -> Result<MiniAppUser> {
        // TODO: Парсинг параметров запуска

        Ok(MiniAppUser {
            id: 0,
            first_name: "User".to_string(),
            last_name: "Test".to_string(),
            photo_url: None,
        })
    }
}

/// Пользователь Mini App.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiniAppUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub photo_url: Option<String>,
}

/// Экраны Mini App.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiniAppScreen {
    /// Главный экран
    Home,

    /// Генератор стихов
    PoetryGenerator,

    /// Обучение языкам
    LanguageLearning,

    /// Интерактивный театр
    InteractiveTheater,

    /// Профиль пользователя
    Profile,

    /// Настройки
    Settings,
}

impl MiniAppScreen {
    pub fn route(&self) -> &'static str {
        match self {
            MiniAppScreen::Home => "/",
            MiniAppScreen::PoetryGenerator => "/poetry",
            MiniAppScreen::LanguageLearning => "/learn",
            MiniAppScreen::InteractiveTheater => "/theater",
            MiniAppScreen::Profile => "/profile",
            MiniAppScreen::Settings => "/settings",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            MiniAppScreen::Home => "Academia Lux",
            MiniAppScreen::PoetryGenerator => "Генератор стихов",
            MiniAppScreen::LanguageLearning => "Обучение языкам",
            MiniAppScreen::InteractiveTheater => "Интерактивный театр",
            MiniAppScreen::Profile => "Профиль",
            MiniAppScreen::Settings => "Настройки",
        }
    }
}
