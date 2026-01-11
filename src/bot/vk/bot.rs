/// VK бот для Academia Lux.

use crate::{Result, Error};
use crate::bot::vk::{VKConfig, VKEvent, VKMessage, VKApi, MessageHandler};
use crate::poetry::Generator as PoetryGenerator;
use crate::learning::LanguageTutor;
use crate::theater::InteractivePlay;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, debug, error};

/// VK бот.
pub struct VKBot {
    config: VKConfig,
    api: Arc<VKApi>,
    poetry_generator: Arc<PoetryGenerator>,
    language_tutor: Arc<LanguageTutor>,
    active_plays: Arc<RwLock<Vec<InteractivePlay>>>,
    message_handler: MessageHandler,
}

impl VKBot {
    /// Создать строитель бота.
    pub fn new(config: VKConfig) -> VKBotBuilder {
        VKBotBuilder::new(config)
    }

    /// Запустить бота.
    pub async fn start(&self) -> Result<()> {
        info!("Starting VK bot for group {}", self.config.group_id);

        // TODO: Реализовать Long Poll или Callback API
        info!("VK bot started successfully");

        Ok(())
    }

    /// Обработать входящее событие.
    pub async fn handle_event(&self, event: VKEvent) -> Result<String> {
        match event {
            VKEvent::Confirmation => {
                info!("Confirmation request received");
                Ok(self.config.confirmation_key.clone()
                    .unwrap_or_else(|| "confirmation_key_not_set".to_string()))
            }

            VKEvent::MessageNew { object } => {
                debug!("New message from {}: {}", object.from_id, object.text);
                self.handle_message(object).await?;
                Ok("ok".to_string())
            }

            VKEvent::MessageEdit { object } => {
                debug!("Message edited: {}", object.id);
                Ok("ok".to_string())
            }

            VKEvent::MessageEvent { object } => {
                debug!("Message event from {}", object.user_id);
                self.handle_callback(object).await?;
                Ok("ok".to_string())
            }
        }
    }

    /// Обработать текстовое сообщение.
    async fn handle_message(&self, message: VKMessage) -> Result<()> {
        let text = message.text.trim();

        // Команды
        if text.starts_with('/') {
            return self.handle_command(&message).await;
        }

        // Обычное сообщение
        self.send_message(
            message.peer_id,
            "Привет! Я бот Academia Lux. Используйте /help для списка команд."
        ).await?;

        Ok(())
    }

    /// Обработать команду.
    async fn handle_command(&self, message: &VKMessage) -> Result<()> {
        let parts: Vec<&str> = message.text.split_whitespace().collect();
        let command = parts[0];

        match command {
            "/start" => {
                self.send_message(
                    message.peer_id,
                    "🎭 Добро пожаловать в Academia Lux!\n\n\
                     Я помогу вам:\n\
                     📝 Создавать стихи\n\
                     📚 Учить древние языки\n\
                     🎬 Участвовать в интерактивном театре\n\n\
                     Используйте /help для списка команд"
                ).await?;
            }

            "/help" => {
                self.send_message(
                    message.peer_id,
                    "📖 Доступные команды:\n\n\
                     /poem <тема> - Создать стихотворение\n\
                     /learn <язык> - Начать урок языка\n\
                     /theater - Запустить интерактивный театр\n\
                     /vote <вариант> - Проголосовать в театре\n\
                     /help - Показать эту справку"
                ).await?;
            }

            "/poem" => {
                let theme = parts[1..].join(" ");
                if theme.is_empty() {
                    self.send_message(
                        message.peer_id,
                        "Укажите тему стихотворения: /poem <тема>"
                    ).await?;
                } else {
                    self.generate_poem(message.peer_id, &theme).await?;
                }
            }

            "/learn" => {
                let language = parts.get(1).unwrap_or(&"greek");
                self.start_lesson(message.peer_id, language).await?;
            }

            "/theater" => {
                self.start_theater(message.peer_id).await?;
            }

            "/vote" => {
                let option = parts[1..].join(" ");
                self.vote_in_theater(message.peer_id, &option).await?;
            }

            _ => {
                self.send_message(
                    message.peer_id,
                    "Неизвестная команда. Используйте /help"
                ).await?;
            }
        }

        Ok(())
    }

    /// Обработать callback кнопки.
    async fn handle_callback(&self, event: crate::bot::vk::VKMessageEvent) -> Result<()> {
        debug!("Callback payload: {:?}", event.payload);

        // TODO: Обработка callback кнопок

        Ok(())
    }

    /// Отправить сообщение.
    async fn send_message(&self, peer_id: i64, text: &str) -> Result<()> {
        self.api.send_message(peer_id, text, None).await
    }

    /// Сгенерировать стихотворение.
    async fn generate_poem(&self, peer_id: i64, theme: &str) -> Result<()> {
        self.send_message(peer_id, "✍️ Создаю стихотворение...").await?;

        // TODO: Реальная генерация через poetry_generator
        let poem = format!(
            "📝 Стихотворение на тему '{}':\n\n\
             Строка первая...\n\
             Строка вторая...\n\
             Строка третья...\n\
             Строка четвёртая...",
            theme
        );

        self.send_message(peer_id, &poem).await?;

        Ok(())
    }

    /// Начать урок языка.
    async fn start_lesson(&self, peer_id: i64, language: &str) -> Result<()> {
        let lang_name = match language {
            "greek" => "древнегреческого",
            "latin" => "латыни",
            "slavonic" => "церковнославянского",
            _ => "древнегреческого",
        };

        self.send_message(
            peer_id,
            &format!("📚 Начинаем урок {} языка!\n\nУрок 1: Алфавит", lang_name)
        ).await?;

        Ok(())
    }

    /// Запустить интерактивный театр.
    async fn start_theater(&self, peer_id: i64) -> Result<()> {
        self.send_message(
            peer_id,
            "🎭 Запускаем интерактивный театр!\n\n\
             Сцена 1: Пролог\n\n\
             Выберите развитие сюжета:\n\
             A) Герой отправляется в путешествие\n\
             B) Герой остаётся дома\n\n\
             Голосуйте: /vote A или /vote B"
        ).await?;

        Ok(())
    }

    /// Проголосовать в театре.
    async fn vote_in_theater(&self, peer_id: i64, option: &str) -> Result<()> {
        self.send_message(
            peer_id,
            &format!("✅ Ваш голос за вариант '{}' учтён!", option)
        ).await?;

        Ok(())
    }
}

/// Строитель VK бота.
pub struct VKBotBuilder {
    config: VKConfig,
}

impl VKBotBuilder {
    pub fn new(config: VKConfig) -> Self {
        Self { config }
    }

    pub async fn build(self) -> Result<VKBot> {
        let api = Arc::new(VKApi::new(self.config.clone()));
        let poetry_generator = Arc::new(PoetryGenerator::new().await?);
        let language_tutor = Arc::new(LanguageTutor::new().await?);
        let active_plays = Arc::new(RwLock::new(Vec::new()));
        let message_handler = MessageHandler::new();

        Ok(VKBot {
            config: self.config,
            api,
            poetry_generator,
            language_tutor,
            active_plays,
            message_handler,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vk_config_creation() {
        let config = VKConfig {
            access_token: "test_token".to_string(),
            group_id: 123456,
            api_version: "5.131".to_string(),
            secret_key: None,
            confirmation_key: Some("test_key".to_string()),
        };

        assert_eq!(config.group_id, 123456);
    }
}
