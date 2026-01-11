/// Стихотворные размеры для Academia Lux.
///
/// Определяет ритмическую структуру стихотворения.

use serde::{Deserialize, Serialize};

/// Стихотворный размер (метр).
///
/// Определяет чередование ударных и безударных слогов.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Meter {
    /// Ямб - безударный-ударный (∪ /)
    /// Пример: "Мороз и солнце; день чудесный!"
    Iambic,

    /// Хорей - ударный-безударный (/ ∪)
    /// Пример: "Буря мглою небо кроет"
    Trochaic,

    /// Дактиль - ударный-безударный-безударный (/ ∪ ∪)
    /// Пример: "Тучки небесные, вечные странники"
    Dactylic,

    /// Анапест - безударный-безударный-ударный (∪ ∪ /)
    /// Пример: "О, весна без конца и без краю"
    Anapestic,

    /// Свободный стих (верлибр) - без строгого размера
    Free,
}

impl Meter {
    /// Получить название размера.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::poetry::Meter;
    ///
    /// assert_eq!(Meter::Iambic.name(), "Ямб");
    /// assert_eq!(Meter::Trochaic.name(), "Хорей");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            Meter::Iambic => "Ямб",
            Meter::Trochaic => "Хорей",
            Meter::Dactylic => "Дактиль",
            Meter::Anapestic => "Анапест",
            Meter::Free => "Свободный стих",
        }
    }

    /// Получить описание размера.
    pub fn description(&self) -> &'static str {
        match self {
            Meter::Iambic => "Безударный-ударный (∪ /). Пример: 'Мороз и солнце; день чудесный!'",
            Meter::Trochaic => "Ударный-безударный (/ ∪). Пример: 'Буря мглою небо кроет'",
            Meter::Dactylic => "Ударный-безударный-безударный (/ ∪ ∪). Пример: 'Тучки небесные'",
            Meter::Anapestic => "Безударный-безударный-ударный (∪ ∪ /). Пример: 'О, весна без конца'",
            Meter::Free => "Свободный стих без строгого размера (верлибр)",
        }
    }

    /// Получить паттерн ударений.
    ///
    /// Возвращает строку с символами:
    /// - `/` - ударный слог
    /// - `∪` - безударный слог
    pub fn pattern(&self) -> &'static str {
        match self {
            Meter::Iambic => "∪ /",
            Meter::Trochaic => "/ ∪",
            Meter::Dactylic => "/ ∪ ∪",
            Meter::Anapestic => "∪ ∪ /",
            Meter::Free => "свободный",
        }
    }

    /// Получить все доступные размеры.
    pub fn all() -> Vec<Meter> {
        vec![
            Meter::Iambic,
            Meter::Trochaic,
            Meter::Dactylic,
            Meter::Anapestic,
            Meter::Free,
        ]
    }

    /// Создать размер из строки.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::poetry::Meter;
    ///
    /// let meter = Meter::from_str("iambic").unwrap();
    /// assert_eq!(meter, Meter::Iambic);
    ///
    /// let meter = Meter::from_str("ямб").unwrap();
    /// assert_eq!(meter, Meter::Iambic);
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "iambic" | "ямб" | "iamb" => Some(Meter::Iambic),
            "trochaic" | "хорей" | "trochee" => Some(Meter::Trochaic),
            "dactylic" | "дактиль" | "dactyl" => Some(Meter::Dactylic),
            "anapestic" | "анапест" | "anapest" => Some(Meter::Anapestic),
            "free" | "свободный" | "верлибр" | "vers libre" => Some(Meter::Free),
            _ => None,
        }
    }

    /// Проверить, является ли размер классическим (не свободным).
    pub fn is_classical(&self) -> bool {
        !matches!(self, Meter::Free)
    }

    /// Получить количество слогов в стопе.
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::poetry::Meter;
    ///
    /// assert_eq!(Meter::Iambic.syllables_per_foot(), 2);
    /// assert_eq!(Meter::Dactylic.syllables_per_foot(), 3);
    /// assert_eq!(Meter::Free.syllables_per_foot(), 0);
    /// ```
    pub fn syllables_per_foot(&self) -> usize {
        match self {
            Meter::Iambic | Meter::Trochaic => 2,
            Meter::Dactylic | Meter::Anapestic => 3,
            Meter::Free => 0,
        }
    }
}

impl Default for Meter {
    fn default() -> Self {
        Meter::Iambic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meter_name() {
        assert_eq!(Meter::Iambic.name(), "Ямб");
        assert_eq!(Meter::Trochaic.name(), "Хорей");
        assert_eq!(Meter::Dactylic.name(), "Дактиль");
    }

    #[test]
    fn test_meter_pattern() {
        assert_eq!(Meter::Iambic.pattern(), "∪ /");
        assert_eq!(Meter::Trochaic.pattern(), "/ ∪");
        assert_eq!(Meter::Dactylic.pattern(), "/ ∪ ∪");
    }

    #[test]
    fn test_meter_from_str() {
        assert_eq!(Meter::from_str("iambic"), Some(Meter::Iambic));
        assert_eq!(Meter::from_str("ямб"), Some(Meter::Iambic));
        assert_eq!(Meter::from_str("хорей"), Some(Meter::Trochaic));
        assert_eq!(Meter::from_str("unknown"), None);
    }

    #[test]
    fn test_is_classical() {
        assert!(Meter::Iambic.is_classical());
        assert!(Meter::Trochaic.is_classical());
        assert!(!Meter::Free.is_classical());
    }

    #[test]
    fn test_syllables_per_foot() {
        assert_eq!(Meter::Iambic.syllables_per_foot(), 2);
        assert_eq!(Meter::Trochaic.syllables_per_foot(), 2);
        assert_eq!(Meter::Dactylic.syllables_per_foot(), 3);
        assert_eq!(Meter::Anapestic.syllables_per_foot(), 3);
        assert_eq!(Meter::Free.syllables_per_foot(), 0);
    }

    #[test]
    fn test_all_meters() {
        let meters = Meter::all();
        assert_eq!(meters.len(), 5);
    }

    #[test]
    fn test_default_meter() {
        let meter = Meter::default();
        assert_eq!(meter, Meter::Iambic);
    }
}
