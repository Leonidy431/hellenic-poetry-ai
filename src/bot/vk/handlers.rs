/// Обработчики сообщений VK бота.

use crate::{Result, Error};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Обработчик сообщений.
pub struct MessageHandler {
    commands: Arc<RwLock<HashMap<String, CommandHandler>>>,
}

impl MessageHandler {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Зарегистрировать команду.
    pub fn register_command(&self, command: &str, handler: CommandHandler) {
        let mut commands = self.commands.write();
        commands.insert(command.to_string(), handler);
    }

    /// Обработать команду.
    pub async fn handle_command(&self, command: &str, args: Vec<String>) -> Result<String> {
        let commands = self.commands.read();

        if let Some(handler) = commands.get(command) {
            handler.execute(args).await
        } else {
            Err(Error::NotFound(format!("Command not found: {}", command)))
        }
    }
}

/// Обработчик команды.
pub struct CommandHandler {
    name: String,
    description: String,
    handler: Arc<dyn Fn(Vec<String>) -> Result<String> + Send + Sync>,
}

impl CommandHandler {
    pub fn new<F>(name: &str, description: &str, handler: F) -> Self
    where
        F: Fn(Vec<String>) -> Result<String> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            handler: Arc::new(handler),
        }
    }

    pub async fn execute(&self, args: Vec<String>) -> Result<String> {
        (self.handler)(args)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_command_handler() {
        let handler = MessageHandler::new();

        handler.register_command(
            "/test",
            CommandHandler::new(
                "test",
                "Test command",
                |_args| Ok("Test response".to_string())
            )
        );

        let result = handler.handle_command("/test", vec![]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test response");
    }
}
