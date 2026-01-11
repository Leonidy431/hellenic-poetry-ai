/// Модуль промптов для AI генерации.
///
/// Содержит готовые промпты для различных задач:
/// - Генерация стихов
/// - Обучение языкам
/// - Театральные сценарии
/// - Православные тексты

use crate::poetry::{PoetryStyle, Language, Meter};

/// Промпты для генерации стихов.
pub struct PoetryPrompts;

impl PoetryPrompts {
    /// Системный промпт для генератора стихов.
    ///
    /// Определяет роль AI как поэта.
    pub fn system_prompt() -> &'static str {
        "Ты - талантливый поэт, владеющий различными стилями и языками. \
         Ты создаёшь красивые, глубокие стихотворения с правильной рифмовкой и ритмикой. \
         Твои стихи эмоциональны, образны и запоминающиеся. \
         Ты знаешь русскую, греческую и болгарскую поэтическую традицию."
    }

    /// Промпт для генерации стихотворения.
    ///
    /// # Аргументы
    ///
    /// * `theme` - Тема стихотворения
    /// * `style` - Стиль стихотворения
    /// * `language` - Язык стихотворения
    /// * `meter` - Стихотворный размер
    /// * `lines` - Количество строк (опционально)
    ///
    /// # Примеры
    ///
    /// ```
    /// use academia_lux::ai::prompts::PoetryPrompts;
    /// use academia_lux::poetry::{PoetryStyle, Language, Meter};
    ///
    /// let prompt = PoetryPrompts::generate_poem(
    ///     "море и свобода",
    ///     PoetryStyle::Lyric,
    ///     Language::Russian,
    ///     Meter::Iambic,
    ///     Some(16)
    /// );
    /// ```
    pub fn generate_poem(
        theme: &str,
        style: PoetryStyle,
        language: Language,
        meter: Meter,
        lines: Option<u32>,
    ) -> String {
        let style_desc = match style {
            PoetryStyle::Lyric => "лирическое, эмоциональное",
            PoetryStyle::Epic => "эпическое, повествовательное",
            PoetryStyle::Dramatic => "драматическое, напряжённое",
            PoetryStyle::Pastoral => "пасторальное, идиллическое",
            PoetryStyle::Satirical => "сатирическое, ироничное",
            PoetryStyle::Philosophical => "философское, глубокое",
        };

        let language_name = language.name();

        let meter_desc = match meter {
            Meter::Iambic => "ямб (безударный-ударный)",
            Meter::Trochaic => "хорей (ударный-безударный)",
            Meter::Dactylic => "дактиль (ударный-безударный-безударный)",
            Meter::Anapestic => "анапест (безударный-безударный-ударный)",
            Meter::Free => "свободный стих (верлибр)",
        };

        let lines_instruction = if let Some(n) = lines {
            format!("Стихотворение должно содержать ровно {} строк.", n)
        } else {
            "Стихотворение должно быть оптимальной длины (12-20 строк).".to_string()
        };

        format!(
            "Напиши {} стихотворение на тему: \"{}\"\n\n\
             Язык: {}\n\
             Стихотворный размер: {}\n\
             {}\n\n\
             Требования:\n\
             - Используй правильную рифмовку (ABAB или AABB)\n\
             - Соблюдай ритмику и размер\n\
             - Создай яркие образы и метафоры\n\
             - Стихотворение должно быть эмоциональным и запоминающимся\n\
             - Не добавляй название или комментарии, только текст стихотворения\n\n\
             Стихотворение:",
            style_desc, theme, language_name, meter_desc, lines_instruction
        )
    }

    /// Промпт для генерации стихотворения на православную тему.
    ///
    /// # Аргументы
    ///
    /// * `theme` - Тема (праздник, святой, событие)
    /// * `language` - Язык
    pub fn generate_orthodox_poem(theme: &str, language: Language) -> String {
        format!(
            "Напиши духовное православное стихотворение на тему: \"{}\"\n\n\
             Язык: {}\n\n\
             Требования:\n\
             - Стихотворение должно быть благоговейным и возвышенным\n\
             - Используй образы из православной традиции\n\
             - Соблюдай рифмовку и ритмику\n\
             - Стихотворение должно вдохновлять и утешать\n\
             - Можешь использовать церковнославянские слова (с переводом в скобках)\n\n\
             Стихотворение:",
            theme,
            language.name()
        )
    }

    /// Промпт для генерации детского стихотворения.
    pub fn generate_children_poem(theme: &str, age: u8) -> String {
        format!(
            "Напиши детское стихотворение для детей  лет на тему: \"{}\"\n\n\
             Требования:\n\
             - Простой и понятный язык\n\
             - Весёлая рифмовка\n\
             - Яркие образы, понятные детям\n\
             - Позитивное настроение\n\
             - 8-12 строк\n\n\
             Стихотворение:",
            age, theme
        )
    }
}

/// Промпты для обучения языкам.
pub struct LanguagePrompts;

impl LanguagePrompts {
    /// Системный промпт для языкового тьютора.
    pub fn system_prompt() -> &'static str {
        "Ты - опытный преподаватель греческого, русского и болгарского языков. \
         Ты обучаешь через погружение в поэзию и культуру. \
         Ты терпелив, понятно объясняешь и адаптируешь материал под уровень ученика."
    }

    /// Промпт для создания урока.
    pub fn create_lesson(language: Language, level: &str, topic: &str) -> String {
        format!(
            "Создай урок {} языка для уровня {} на тему: \"{}\"\n\n\
             Урок должен включать:\n\
             1. 10-15 новых слов с переводом\n\
             2. Простые примеры использования\n\
             3. Короткое стихотворение (4-8 строк) с этими словами\n\
             4. 3-5 упражнений для закрепления\n\n\
             Формат:\n\
             ## Новые слова\n\
             [слово] - [перевод] - [пример]\n\n\
             ## Стихотворение\n\
             [текст с переводом сложных слов в скобках]\n\n\
             ## Упражнения\n\
             [упражнения]",
            language.name(),
            level,
            topic
        )
    }
}

/// Промпты для театральных постановок.
pub struct TheaterPrompts;

impl TheaterPrompts {
    /// Системный промпт для театрального сценариста.
    pub fn system_prompt() -> &'static str {
        "Ты - талантливый драматург и сценарист. \
         Ты создаёшь увлекательные интерактивные театральные постановки. \
         Твои сценарии динамичны, эмоциональны и вовлекают зрителей."
    }

    /// Промпт для создания сцены.
    pub fn create_scene(
        title: &str,
        genre: &str,
        previous_choice: Option<&str>,
    ) -> String {
        let context = if let Some(choice) = previous_choice {
            format!("Предыдущий выбор зрителей: {}\n", choice)
        } else {
            String::new()
        };

        format!(
            "Создай театральную сцену для постановки \"{}\" в жанре {}.\n\
             {}\n\
             Сцена должна включать:\n\
             1. Описание места действия\n\
             2. Диалоги персонажей (2-3 персонажа)\n\
             3. Два варианта развития сюжета для голосования зрителей\n\n\
             Формат:\n\
             ## Место действия\n\
             [описание]\n\n\
             ## Диалоги\n\
             [имя]: [реплика]\n\n\
             ## Варианты развития\n\
             A) [вариант 1]\n\
             B) [вариант 2]",
            title, genre, context
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poetry_system_prompt() {
        let prompt = PoetryPrompts::system_prompt();
        assert!(prompt.contains("поэт"));
        assert!(prompt.contains("стихотворения"));
    }

    #[test]
    fn test_generate_poem_prompt() {
        let prompt = PoetryPrompts::generate_poem(
            "море",
            PoetryStyle::Lyric,
            Language::Russian,
            Meter::Iambic,
            Some(16),
        );

        assert!(prompt.contains("море"));
        assert!(prompt.contains("лирическое"));
        assert!(prompt.contains("ямб"));
        assert!(prompt.contains("16 строк"));
    }

    #[test]
    fn test_language_prompts() {
        let prompt = LanguagePrompts::create_lesson(
            Language::Greek,
            "A1",
            "приветствия",
        );

        assert!(prompt.contains("Ελληνικά"));
        assert!(prompt.contains("A1"));
        assert!(prompt.contains("приветствия"));
    }
}
