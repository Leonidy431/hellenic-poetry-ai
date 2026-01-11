/// Модуль ботов и интеграций.
///
/// Поддерживает:
/// - Telegram бот
/// - VK бот и Mini App
/// - Яндекс Алиса
/// - Discord бот

pub mod vk;
// pub mod telegram;
// pub mod alice;
// pub mod discord;

pub use vk::{VKBot, VKConfig, VKMiniApp, MiniAppConfig};
