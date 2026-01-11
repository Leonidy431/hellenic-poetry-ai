/// Модуль интеграции с ВКонтакте.
///
/// Предоставляет фронтенд через VK Mini Apps и VK Bots API.
///
/// # Возможности
///
/// - VK Mini App для интерактивного театра
/// - VK Bot для обучения языкам
/// - Интеграция с VK Pay для монетизации
/// - VK Stories для публикации стихов
/// - VK Live для прямых трансляций театра
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::bot::vk::{VKBot, VKConfig};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = VKConfig {
///         access_token: "your_token".to_string(),
///         group_id: 123456789,
///         api_version: "5.131".to_string(),
///     };
///
///     let bot = VKBot::new(config).await?;
///     bot.start().await?;
///
///     Ok(())
/// }
/// ```

mod bot;
mod mini_app;
mod api;
mod handlers;
mod keyboard;

pub use bot::{VKBot, VKBotBuilder};
pub use mini_app::{VKMiniApp, MiniAppConfig};
pub use api::{VKApi, VKApiClient};
pub use handlers::{MessageHandler, CommandHandler};
pub use keyboard::{Keyboard, Button, ButtonColor};

use serde::{Deserialize, Serialize};

/// Конфигурация VK бота.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKConfig {
    /// Токен доступа группы
    pub access_token: String,

    /// ID группы
    pub group_id: i64,

    /// Версия API
    pub api_version: String,

    /// Секретный ключ для Callback API
    pub secret_key: Option<String>,

    /// Ключ подтверждения сервера
    pub confirmation_key: Option<String>,
}

impl VKConfig {
    /// Создать конфигурацию из переменных окружения.
    pub fn from_env() -> crate::Result<Self> {
        Ok(Self {
            access_token: std::env::var("VK_ACCESS_TOKEN")
                .map_err(|_| crate::Error::ConfigError("VK_ACCESS_TOKEN not set".to_string()))?,
            group_id: std::env::var("VK_GROUP_ID")
                .map_err(|_| crate::Error::ConfigError("VK_GROUP_ID not set".to_string()))?
                .parse()
                .map_err(|_| crate::Error::ConfigError("Invalid VK_GROUP_ID".to_string()))?,
            api_version: std::env::var("VK_API_VERSION")
                .unwrap_or_else(|_| "5.131".to_string()),
            secret_key: std::env::var("VK_SECRET_KEY").ok(),
            confirmation_key: std::env::var("VK_CONFIRMATION_KEY").ok(),
        })
    }
}

/// Тип события VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VKEvent {
    /// Подтверждение сервера
    #[serde(rename = "confirmation")]
    Confirmation,

    /// Новое сообщение
    #[serde(rename = "message_new")]
    MessageNew { object: VKMessage },

    /// Редактирование сообщения
    #[serde(rename = "message_edit")]
    MessageEdit { object: VKMessage },

    /// Callback кнопки
    #[serde(rename = "message_event")]
    MessageEvent { object: VKMessageEvent },
}

/// Сообщение VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKMessage {
    pub id: i64,
    pub date: i64,
    pub peer_id: i64,
    pub from_id: i64,
    pub text: String,
    pub attachments: Vec<VKAttachment>,
    pub payload: Option<String>,
}

/// Вложение VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKAttachment {
    #[serde(rename = "type")]
    pub attachment_type: String,
    pub photo: Option<VKPhoto>,
    pub video: Option<VKVideo>,
    pub audio: Option<VKAudio>,
    pub doc: Option<VKDocument>,
}

/// Фото VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKPhoto {
    pub id: i64,
    pub owner_id: i64,
    pub sizes: Vec<VKPhotoSize>,
}

/// Размер фото.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKPhotoSize {
    #[serde(rename = "type")]
    pub size_type: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

/// Видео VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKVideo {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub duration: u32,
}

/// Аудио VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKAudio {
    pub id: i64,
    pub owner_id: i64,
    pub artist: String,
    pub title: String,
    pub duration: u32,
}

/// Документ VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKDocument {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub size: u64,
    pub url: String,
}

/// Событие callback кнопки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKMessageEvent {
    pub user_id: i64,
    pub peer_id: i64,
    pub event_id: String,
    pub payload: serde_json::Value,
    pub conversation_message_id: i64,
}

/// Ответ на событие.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKEventResponse {
    #[serde(rename = "type")]
    pub response_type: String,
}

impl VKEventResponse {
    /// Ответ "ok".
    pub fn ok() -> Self {
        Self {
            response_type: "ok".to_string(),
        }
    }

    /// Ответ с ключом подтверждения.
    pub fn confirmation(key: &str) -> String {
        key.to_string()
    }
}
