/// Стили стихотворений для Academia Lux.
///
/// Определяет различные поэтические стили и жанры.

use serde::{Deserialize, Serialize};

/// Стиль стихотворения.
///
/// Определяет жанр и настроение стихотворения.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoetryStyle {
    /// Лирическое стихотворение - выражение чувств и эмоций.
    Lyric,

    /// Эпическое стихотворение - повествование о событиях.
    Epic,

    /// Драматическое стихотворение - напряжённое, конфликтное.
    Dramatic,

    /// Пасторальное стихотворение - идиллия, природа.
    Pastoral,

    /// Сатирическое стихотворение - ирония, критика.
    Satirical,

    /// Философское стихотворение - размышления о жизни.
    Philosophical,
}

impl PoetryStyle {
    /// Получить название стиля.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::poetry::PoetryStyle;
    ///
    /// assert_eq!(PoetryStyle::Lyric.name(), "Лирическое");
    /// assert_eq!(PoetryStyle::Epic.name(), "Эпическое");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            PoetryStyle::Lyric => "Лирическое",
            PoetryStyle::Epic => "Эпическое",
            PoetryStyle::Dramatic => "Драматическое",
            PoetryStyle::Pastoral => "Пасторальное",
            PoetryStyle::Satirical => "Сатирическое",
            PoetryStyle::Philosophical => "Философское",
        }
    }

    /// Получить описание стиля.
    pub fn description(&self) -> &'static str {
        match self {
            PoetryStyle::Lyric => "Выражение личных чувств и эмоций",
            PoetryStyle::Epic => "Повествование о героических событиях",
            PoetryStyle::Dramatic => "Напряжённое, конфликтное повествование",
            PoetryStyle::Pastoral => "Идиллическое описание природы и сельской жизни",
            PoetryStyle::Satirical => "Ироническая критика пороков общества",
            PoetryStyle::Philosophical => "Глубокие размышления о смысле жизни",
        }
    }

    /// Получить все доступные стили.
    pub fn all() -> Vec<PoetryStyle> {
        vec![
            PoetryStyle::Lyric,
            PoetryStyle::Epic,
            PoetryStyle::Dramatic,
            PoetryStyle::Pastoral,
            PoetryStyle::Satirical,
            PoetryStyle::Philosophical,
        ]
    }

    /// Создать стиль из строки.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::poetry::PoetryStyle;
    ///
    /// let style = PoetryStyle::from_str("lyric").unwrap();
    /// assert_eq!(style, PoetryStyle::Lyric);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lyric" | "лирическое" => Some(PoetryStyle::Lyric),
            "epic" | "эпическое" => Some(PoetryStyle::Epic),
            "dramatic" | "драматическое" => Some(PoetryStyle::Dramatic),
            "pastoral" | "пасторальное" => Some(PoetryStyle::Pastoral),
            "satirical" | "сатирическое" => Some(PoetryStyle::Satirical),
            "philosophical" | "философское" => Some(PoetryStyle::Philosophical),
            _ => None,
        }
    }
}

impl Default for PoetryStyle {
    fn default() -> Self {
        PoetryStyle::Lyric
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poetry_style_name() {
        assert_eq!(PoetryStyle::Lyric.name(), "Лирическое");
        assert_eq!(PoetryStyle::Epic.name(), "Эпическое");
    }

    #[test]
    fn test_poetry_style_from_str() {
        assert_eq!(
            PoetryStyle::from_str("lyric"),
            Some(PoetryStyle::Lyric)
        );
        assert_eq!(
            PoetryStyle::from_str("лирическое"),
            Some(PoetryStyle::Lyric)
        );
        assert_eq!(PoetryStyle::from_str("unknown"), None);
    }

    #[test]
    fn test_all_styles() {
        let styles = PoetryStyle::all();
        assert_eq!(styles.len(), 6);
    }

    #[test]
    fn test_default_style() {
        let style = PoetryStyle::default();
        assert_eq!(style, PoetryStyle::Lyric);
    }
}
