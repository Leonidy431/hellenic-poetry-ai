/// Клавиатура VK для ботов.

use serde::{Deserialize, Serialize};

/// Клавиатура VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyboard {
    pub one_time: bool,
    pub inline: bool,
    pub buttons: Vec<Vec<Button>>,
}

impl Keyboard {
    /// Создать новую клавиатуру.
    pub fn new() -> Self {
        Self {
            one_time: false,
            inline: false,
            buttons: vec![],
        }
    }

    /// Одноразовая клавиатура (скрывается после нажатия).
    pub fn one_time(mut self) -> Self {
        self.one_time = true;
        self
    }

    /// Inline клавиатура (прикреплена к сообщению).
    pub fn inline(mut self) -> Self {
        self.inline = true;
        self
    }

    /// Добавить ряд кнопок.
    pub fn add_row(mut self, buttons: Vec<Button>) -> Self {
        self.buttons.push(buttons);
        self
    }

    /// Добавить одну кнопку в новый ряд.
    pub fn add_button(mut self, button: Button) -> Self {
        self.buttons.push(vec![button]);
        self
    }

    /// Создать клавиатуру с текстовыми кнопками.
    pub fn text_buttons(labels: Vec<&str>) -> Self {
        let mut keyboard = Self::new();

        for label in labels {
            keyboard = keyboard.add_button(Button::text(label));
        }

        keyboard
    }

    /// Создать inline клавиатуру с callback кнопками.
    pub fn callback_buttons(buttons: Vec<(&str, &str)>) -> Self {
        let mut keyboard = Self::new().inline();

        for (label, payload) in buttons {
            keyboard = keyboard.add_button(Button::callback(label, payload));
        }

        keyboard
    }
}

/// Кнопка клавиатуры.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    pub action: ButtonAction,
    pub color: Option<ButtonColor>,
}

impl Button {
    /// Текстовая кнопка.
    pub fn text(label: &str) -> Self {
        Self {
            action: ButtonAction::Text {
                label: label.to_string(),
                payload: None,
            },
            color: Some(ButtonColor::Primary),
        }
    }

    /// Callback кнопка.
    pub fn callback(label: &str, payload: &str) -> Self {
        Self {
            action: ButtonAction::Callback {
                label: label.to_string(),
                payload: payload.to_string(),
            },
            color: Some(ButtonColor::Primary),
        }
    }

    /// Кнопка с ссылкой.
    pub fn link(label: &str, url: &str) -> Self {
        Self {
            action: ButtonAction::OpenLink {
                label: label.to_string(),
                link: url.to_string(),
            },
            color: None,
        }
    }

    /// Установить цвет кнопки.
    pub fn color(mut self, color: ButtonColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// Действие кнопки.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ButtonAction {
    /// Текстовая кнопка
    #[serde(rename = "text")]
    Text {
        label: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<String>,
    },

    /// Callback кнопка
    #[serde(rename = "callback")]
    Callback {
        label: String,
        payload: String,
    },

    /// Кнопка с ссылкой
    #[serde(rename = "open_link")]
    OpenLink {
        label: String,
        link: String,
    },

    /// Кнопка локации
    #[serde(rename = "location")]
    Location,

    /// Кнопка VK Pay
    #[serde(rename = "vkpay")]
    VKPay {
        hash: String,
    },

    /// Кнопка VK Apps
    #[serde(rename = "open_app")]
    OpenApp {
        app_id: i64,
        owner_id: i64,
        label: String,
        hash: String,
    },
}

/// Цвет кнопки.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ButtonColor {
    /// Синяя (основная)
    Primary,

    /// Белая (второстепенная)
    Secondary,

    /// Красная (опасная)
    Negative,

    /// Зелёная (положительная)
    Positive,
}

impl ButtonColor {
    pub fn name_ru(&self) -> &'static str {
        match self {
            ButtonColor::Primary => "Синяя",
            ButtonColor::Secondary => "Белая",
            ButtonColor::Negative => "Красная",
            ButtonColor::Positive => "Зелёная",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_creation() {
        let keyboard = Keyboard::new()
            .add_button(Button::text("Кнопка 1"))
            .add_button(Button::text("Кнопка 2"));

        assert_eq!(keyboard.buttons.len(), 2);
    }

    #[test]
    fn test_text_buttons() {
        let keyboard = Keyboard::text_buttons(vec!["A", "B", "C"]);
        assert_eq!(keyboard.buttons.len(), 3);
    }

    #[test]
    fn test_callback_buttons() {
        let keyboard = Keyboard::callback_buttons(vec![
            ("Вариант A", "option_a"),
            ("Вариант B", "option_b"),
        ]);

        assert!(keyboard.inline);
        assert_eq!(keyboard.buttons.len(), 2);
    }
}
