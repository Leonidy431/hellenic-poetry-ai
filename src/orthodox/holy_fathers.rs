/// Модуль работы с учением святых отцов.
///
/// Работает как **цензор и наставник** для проверки контента:
/// - Проверяет сюжеты театральных постановок на соответствие православию
/// - Проверяет стихи на духовную правильность
/// - Предоставляет мудрость и правила для создания произведений
/// - Направляет развитие сюжета в правильное русло
///
/// НЕ используется для прямого цитирования в стихах!
/// Используется как **внутренний фильтр и советник**.

use crate::{Result, Error};
use crate::orthodox::HolyFatherQuote;
use std::collections::HashMap;
use tracing::{info, debug};

/// Модуль работы с высказываниями святых отцов.
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::orthodox::HolyFathersModule;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let module = HolyFathersModule::new().await?;
///
///     // Поиск высказываний по теме
///     let quotes = module.find_by_theme("любовь").await?;
///
///     for quote in quotes {
///         println!("{}: {}", quote.saint_name, quote.quote);
///     }
///
///     Ok(())
/// }
/// ```
pub struct HolyFathersModule {
    /// База высказываний святых отцов.
    quotes: Vec<HolyFatherQuote>,

    /// Индекс по темам для быстрого поиска.
    theme_index: HashMap<String, Vec<usize>>,

    /// Индекс по святым.
    saint_index: HashMap<String, Vec<usize>>,
}

impl HolyFathersModule {
    /// Создать новый модуль.
    ///
    /// Загружает базу высказываний святых отцов.
    pub async fn new() -> Result<Self> {
        info!("Initializing Holy Fathers module");

        // Загрузка базы высказываний
        let quotes = Self::load_default_quotes();

        // Построение индексов
        let mut theme_index: HashMap<String, Vec<usize>> = HashMap::new();
        let mut saint_index: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, quote) in quotes.iter().enumerate() {
            // Индекс по теме
            theme_index
                .entry(quote.theme.to_lowercase())
                .or_insert_with(Vec::new)
                .push(idx);

            // Индекс по святому
            saint_index
                .entry(quote.saint_name.to_lowercase())
                .or_insert_with(Vec::new)
                .push(idx);

            // Индекс по тегам
            for tag in &quote.tags {
                theme_index
                    .entry(tag.to_lowercase())
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }

        info!("Holy Fathers module initialized with {} quotes", quotes.len());

        Ok(Self {
            quotes,
            theme_index,
            saint_index,
        })
    }

    /// Найти высказывания по теме.
    ///
    /// # Аргументы
    ///
    /// * `theme` - Тема для поиска (любовь, смирение, молитва и т.д.)
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::orthodox::HolyFathersModule;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let module = HolyFathersModule::new().await?;
    /// let quotes = module.find_by_theme("любовь").await?;
    /// println!("Найдено {} высказываний о любви", quotes.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_by_theme(&self, theme: &str) -> Result<Vec<HolyFatherQuote>> {
        debug!("Searching quotes by theme: {}", theme);

        let theme_lower = theme.to_lowercase();

        let indices = self
            .theme_index
            .get(&theme_lower)
            .ok_or_else(|| Error::NotFound(format!("No quotes found for theme: {}", theme)))?;

        let quotes: Vec<HolyFatherQuote> = indices
            .iter()
            .filter_map(|&idx| self.quotes.get(idx).cloned())
            .collect();

        debug!("Found {} quotes for theme: {}", quotes.len(), theme);

        Ok(quotes)
    }

    /// Найти высказывания конкретного святого.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::orthodox::HolyFathersModule;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let module = HolyFathersModule::new().await?;
    /// let quotes = module.find_by_saint("Иоанн Златоуст").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_by_saint(&self, saint_name: &str) -> Result<Vec<HolyFatherQuote>> {
        debug!("Searching quotes by saint: {}", saint_name);

        let saint_lower = saint_name.to_lowercase();

        let indices = self
            .saint_index
            .get(&saint_lower)
            .ok_or_else(|| Error::NotFound(format!("No quotes found for saint: {}", saint_name)))?;

        let quotes: Vec<HolyFatherQuote> = indices
            .iter()
            .filter_map(|&idx| self.quotes.get(idx).cloned())
            .collect();

        debug!("Found {} quotes for saint: {}", quotes.len(), saint_name);

        Ok(quotes)
    }

    /// Получить случайное высказывание по теме.
    ///
    /// Используется для вдохновения при создании контента.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::orthodox::HolyFathersModule;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let module = HolyFathersModule::new().await?;
    /// let quote = module.get_random_by_theme("смирение").await?;
    /// println!("{}: {}", quote.saint_name, quote.quote);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_random_by_theme(&self, theme: &str) -> Result<HolyFatherQuote> {
        use rand::seq::SliceRandom;

        let quotes = self.find_by_theme(theme).await?;

        quotes
            .choose(&mut rand::thread_rng())
            .cloned()
            .ok_or_else(|| Error::NotFound(format!("No quotes found for theme: {}", theme)))
    }

    /// Получить все доступные темы.
    pub fn get_all_themes(&self) -> Vec<String> {
        self.theme_index.keys().cloned().collect()
    }

    /// Получить всех святых отцов в базе.
    pub fn get_all_saints(&self) -> Vec<String> {
        self.saint_index.keys().cloned().collect()
    }

    /// **ЦЕНЗОР**: Проверить сюжет театральной постановки на соответствие православию.
    ///
    /// Анализирует сюжет и возвращает рекомендации по улучшению.
    ///
    /// # Аргументы
    ///
    /// * `plot` - Текст сюжета для проверки
    ///
    /// # Возвращает
    ///
    /// Результат проверки с рекомендациями
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::orthodox::HolyFathersModule;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let module = HolyFathersModule::new().await?;
    /// let result = module.check_plot_orthodoxy(
    ///     "Сюжет о борьбе добра и зла..."
    /// ).await?;
    ///
    /// if result.is_approved {
    ///     println!("Сюжет одобрен!");
    /// } else {
    ///     println!("Рекомендации: {}", result.recommendations);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_plot_orthodoxy(&self, plot: &str) -> Result<OrthodoxCheckResult> {
        debug!("Checking plot orthodoxy: {} chars", plot.len());

        // TODO: Реализовать полную проверку через AI с использованием учения святых отцов
        // Сейчас - базовая проверка

        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Проверка на негативные темы
        let negative_keywords = ["месть", "ненависть", "гордыня без покаяния"];
        for keyword in negative_keywords {
            if plot.to_lowercase().contains(keyword) {
                issues.push(format!("Обнаружена тема: {}", keyword));
                recommendations.push(format!(
                    "Рассмотрите добавление темы покаяния и преображения для '{}'",
                    keyword
                ));
            }
        }

        // Проверка на позитивные темы
        let positive_keywords = ["любовь", "прощение", "смирение", "вера", "надежда"];
        let has_positive = positive_keywords
            .iter()
            .any(|k| plot.to_lowercase().contains(k));

        if !has_positive {
            recommendations.push(
                "Рекомендуется добавить темы любви, прощения или смирения".to_string()
            );
        }

        let is_approved = issues.is_empty();

        Ok(OrthodoxCheckResult {
            is_approved,
            issues,
            recommendations,
            suggested_themes: self.suggest_themes_for_plot(plot).await?,
        })
    }

    /// **ЦЕНЗОР**: Проверить стихотворение на духовную правильность.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::orthodox::HolyFathersModule;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let module = HolyFathersModule::new().await?;
    /// let result = module.check_poem_orthodoxy(
    ///     "Стихотворение о любви к Богу..."
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_poem_orthodoxy(&self, poem: &str) -> Result<OrthodoxCheckResult> {
        debug!("Checking poem orthodoxy: {} chars", poem.len());

        // Аналогичная проверка для стихов
        self.check_plot_orthodoxy(poem).await
    }

    /// Предложить темы для развития сюжета на основе учения святых отцов.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::orthodox::HolyFathersModule;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let module = HolyFathersModule::new().await?;
    /// let themes = module.suggest_themes_for_plot(
    ///     "История о человеке, потерявшем веру..."
    /// ).await?;
    ///
    /// for theme in themes {
    ///     println!("Рекомендуемая тема: {}", theme);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn suggest_themes_for_plot(&self, plot: &str) -> Result<Vec<String>> {
        let mut themes = Vec::new();

        // Анализ сюжета и предложение тем
        if plot.to_lowercase().contains("потер") || plot.to_lowercase().contains("утрат") {
            themes.push("Обретение веры через испытания".to_string());
            themes.push("Божественное Провидение".to_string());
        }

        if plot.to_lowercase().contains("борьба") || plot.to_lowercase().contains("конфликт") {
            themes.push("Духовная брань и победа через смирение".to_string());
            themes.push("Прощение врагов".to_string());
        }

        if plot.to_lowercase().contains("любовь") {
            themes.push("Жертвенная любовь по образу Христа".to_string());
            themes.push("Любовь к ближнему".to_string());
        }

        // Если тем не найдено, предложить универсальные
        if themes.is_empty() {
            themes.push("Путь к Богу через покаяние".to_string());
            themes.push("Преображение души".to_string());
            themes.push("Сила молитвы".to_string());
        }

        Ok(themes)
    }

    /// Получить правила создания православных произведений.
    ///
    /// Возвращает руководство по созданию духовно правильного контента.
    pub fn get_creation_guidelines(&self) -> OrthodoxCreationGuidelines {
        OrthodoxCreationGuidelines {
            core_principles: vec![
                "Произведение должно вести к Богу, а не от Него".to_string(),
                "Зло должно быть показано как путь к страданию".to_string(),
                "Добро должно побеждать через смирение и любовь".to_string(),
                "Покаяние и преображение - ключевые темы".to_string(),
            ],
            forbidden_themes: vec![
                "Оправдание греха".to_string(),
                "Богохульство".to_string(),
                "Пропаганда порока".to_string(),
            ],
            recommended_themes: vec![
                "Любовь к Богу и ближнему".to_string(),
                "Смирение и кротость".to_string(),
                "Покаяние и прощение".to_string(),
                "Вера, надежда, любовь".to_string(),
                "Жертвенность".to_string(),
            ],
            plot_structure: vec![
                "Начало: показать состояние героя".to_string(),
                "Развитие: испытания и духовная борьба".to_string(),
                "Кульминация: момент выбора между добром и злом".to_string(),
                "Развязка: преображение или покаяние".to_string(),
            ],
        }
    }

    /// Загрузить базу высказываний по умолчанию.
    ///
    /// TODO: В будущем загружать из файла или базы данных.
    fn load_default_quotes() -> Vec<HolyFatherQuote> {
        vec![
            // Иоанн Златоуст
            HolyFatherQuote::new(
                "Иоанн Златоуст",
                "Любовь есть корень, источник и мать всех благ",
                "любовь"
            )
            .with_source("Беседы на Евангелие от Матфея")
            .with_tags(vec!["любовь".to_string(), "добродетель".to_string()]),

            HolyFatherQuote::new(
                "Иоанн Златоуст",
                "Молитва есть свет души, истинное познание Бога",
                "молитва"
            )
            .with_source("О молитве")
            .with_tags(vec!["молитва".to_string(), "богопознание".to_string()]),

            // Василий Великий
            HolyFatherQuote::new(
                "Василий Великий",
                "Смирение есть основание всех добродетелей",
                "смирение"
            )
            .with_source("Беседы")
            .with_tags(vec!["смирение".to_string(), "добродетель".to_string()]),

            HolyFatherQuote::new(
                "Василий Великий",
                "Пост есть оружие, уготованное Богом",
                "пост"
            )
            .with_source("О посте")
            .with_tags(vec!["пост".to_string(), "аскетика".to_string()]),

            // Григорий Богослов
            HolyFatherQuote::new(
                "Григорий Богослов",
                "Вера без дел мертва, как и дела без веры",
                "вера"
            )
            .with_source("Слова")
            .with_tags(vec!["вера".to_string(), "дела".to_string()]),

            // Антоний Великий
            HolyFatherQuote::new(
                "Антоний Великий",
                "Кто познал немощь свою, тот достиг совершенства",
                "смирение"
            )
            .with_source("Изречения")
            .with_tags(vec!["смирение".to_string(), "самопознание".to_string()]),

            // Исаак Сирин
            HolyFatherQuote::new(
                "Исаак Сирин",
                "Любовь к Богу рождается от беседы с Ним",
                "любовь"
            )
            .with_source("Слова подвижнические")
            .with_tags(vec!["любовь".to_string(), "молитва".to_string()]),

            HolyFatherQuote::new(
                "Исаак Сирин",
                "Безмолвие есть таинство будущего века",
                "безмолвие"
            )
            .with_source("Слова подвижнические")
            .with_tags(vec!["безмолвие".to_string(), "исихазм".to_string()]),

            // Иоанн Лествичник
            HolyFatherQuote::new(
                "Иоанн Лествичник",
                "Начало очищения души - познание своих грехов",
                "покаяние"
            )
            .with_source("Лествица")
            .with_tags(vec!["покаяние".to_string(), "самопознание".to_string()]),

            // Ефрем Сирин
            HolyFatherQuote::new(
                "Ефрем Сирин",
                "Слезы покаяния омывают душу от скверны греха",
                "покаяние"
            )
            .with_source("Творения")
            .with_tags(vec!["покаяние".to_string(), "слезы".to_string()]),

            // Макарий Египетский
            HolyFatherQuote::new(
                "Макарий Египетский",
                "Сердце управляет всем составом человека",
                "сердце"
            )
            .with_source("Духовные беседы")
            .with_tags(vec!["сердце".to_string(), "внутренняя жизнь".to_string()]),

            // Игнатий Брянчанинов
            HolyFatherQuote::new(
                "Игнатий Брянчанинов",
                "Смирение есть единственная жертва, которую требует от нас Бог",
                "смирение"
            )
            .with_source("Аскетические опыты")
            .with_tags(vec!["смирение".to_string(), "жертва".to_string()]),

            // Феофан Затворник
            HolyFatherQuote::new(
                "Феофан Затворник",
                "Молитва есть дыхание духа",
                "молитва"
            )
            .with_source("Письма о духовной жизни")
            .with_tags(vec!["молитва".to_string(), "духовная жизнь".to_string()]),

            // Серафим Саровский
            HolyFatherQuote::new(
                "Серафим Саровский",
                "Стяжи дух мирен, и тысячи вокруг тебя спасутся",
                "мир"
            )
            .with_source("Наставления")
            .with_tags(vec!["мир".to_string(), "спасение".to_string()]),

            HolyFatherQuote::new(
                "Серафим Саровский",
                "Радость моя, Христос воскресе!",
                "радость"
            )
            .with_source("Наставления")
            .with_tags(vec!["радость".to_string(), "воскресение".to_string()]),

            // Паисий Святогорец
            HolyFatherQuote::new(
                "Паисий Святогорец",
                "Добрые помыслы - это ангелы, злые - бесы",
                "помыслы"
            )
            .with_source("Слова")
            .with_tags(vec!["помыслы".to_string(), "духовная брань".to_string()]),

            // Иоанн Кронштадтский
            HolyFatherQuote::new(
                "Иоанн Кронштадтский",
                "Живи для других, тогда и другие будут жить для тебя",
                "любовь"
            )
            .with_source("Моя жизнь во Христе")
            .with_tags(vec!["любовь".to_string(), "служение".to_string()]),

            // Амвросий Оптинский
            HolyFatherQuote::new(
                "Амвросий Оптинский",
                "Жить - не тужить, никого не осуждать, никому не досаждать, и всем мое почтение",
                "жизнь"
            )
            .with_source("Письма")
            .with_tags(vec!["жизнь".to_string(), "мудрость".to_string()]),

            // Силуан Афонский
            HolyFatherQuote::new(
                "Силуан Афонский",
                "Держи ум свой во аде и не отчаивайся",
                "смирение"
            )
            .with_source("Писания")
            .with_tags(vec!["смирение".to_string(), "надежда".to_string()]),

            // Иустин Попович
            HolyFatherQuote::new(
                "Иустин Попович",
                "Православие есть Богочеловечество",
                "православие"
            )
            .with_source("Догматика")
            .with_tags(vec!["православие".to_string(), "богословие".to_string()]),
        ]
    }
}

/// Результат проверки контента на православие.
#[derive(Debug, Clone)]
pub struct OrthodoxCheckResult {
    /// Одобрен ли контент.
    pub is_approved: bool,

    /// Обнаруженные проблемы.
    pub issues: Vec<String>,

    /// Рекомендации по улучшению.
    pub recommendations: Vec<String>,

    /// Предложенные темы для развития.
    pub suggested_themes: Vec<String>,
}

/// Руководство по созданию православных произведений.
#[derive(Debug, Clone)]
pub struct OrthodoxCreationGuidelines {
    /// Основные принципы.
    pub core_principles: Vec<String>,

    /// Запрещённые темы.
    pub forbidden_themes: Vec<String>,

    /// Рекомендуемые темы.
    pub recommended_themes: Vec<String>,

    /// Структура сюжета.
    pub plot_structure: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_holy_fathers_module() {
        let module = HolyFathersModule::new().await.unwrap();

        // Тест поиска по теме
        let quotes = module.find_by_theme("любовь").await.unwrap();
        assert!(!quotes.is_empty());

        // Тест поиска по святому
        let quotes = module.find_by_saint("Иоанн Златоуст").await.unwrap();
        assert!(!quotes.is_empty());

        // Тест получения случайного высказывания
        let quote = module.get_random_by_theme("смирение").await.unwrap();
        assert!(!quote.quote.is_empty());
    }

    #[tokio::test]
    async fn test_get_all_themes() {
        let module = HolyFathersModule::new().await.unwrap();
        let themes = module.get_all_themes();
        assert!(!themes.is_empty());
    }
}
