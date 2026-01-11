/// Генератор стихотворений для Academia Lux.
///
/// Использует AI для создания стихотворений на различных языках.
///
/// # Примеры
///
/// ```no_run
/// use academia_lux::poetry::{Generator, PoetryStyle, Language, Meter};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let generator = Generator::new().await?;
///
///     let poem = generator
///         .generate()
///         .theme("море и свобода")
///         .style(PoetryStyle::Lyric)
///         .language(Language::Russian)
///         .meter(Meter::Iambic)
///         .lines(16)
///         .await?;
///
///     println!("Название: {}", poem.title);
///     println!("\n{}", poem.content);
///     Ok(())
/// }
/// ```

use crate::{Result, Error, Config};
use crate::ai::{AiClient, AiProvider};
use crate::ai::prompts::PoetryPrompts;
use crate::poetry::{Poem, Language, PoetryStyle, Meter};
use tracing::{info, debug};

/// Генератор стихотворений.
///
/// Использует AI (OpenAI или Claude) для создания стихотворений.
pub struct Generator {
    /// AI клиент для генерации.
    ai_client: AiClient,
}

impl Generator {
    /// Создать новый генератор.
    ///
    /// Использует конфигурацию по умолчанию.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если не удалось инициализировать AI клиент.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// use academia_lux::poetry::Generator;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let generator = Generator::new().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new() -> Result<Self> {
        // Загрузка конфигурации
        let config = Config::default();

        // Определение провайдера
        let provider = AiProvider::from_str(&config.ai.provider)
            .ok_or_else(|| Error::Config(format!("Unknown AI provider: {}", config.ai.provider)))?;

        // Получение API ключа
        let api_key = match provider {
            AiProvider::OpenAI => {
                config.ai.openai_api_key
                    .or_else(|| std::env::var("OPENAI_API_KEY").ok())
                    .ok_or_else(|| Error::Config("OpenAI API key not found".to_string()))?
            }
            AiProvider::Claude => {
                config.ai.anthropic_api_key
                    .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
                    .ok_or_else(|| Error::Config("Anthropic API key not found".to_string()))?
            }
        };

        // Создание AI клиента
        let ai_client = AiClient::new(provider, &api_key).await?;

        info!("Poetry generator initialized with provider: {}", ai_client.provider_name());

        Ok(Self { ai_client })
    }

    /// Создать генератор с кастомным AI клиентом.
    ///
    /// # Аргументы
    ///
    /// * `ai_client` - AI клиент для использования
    pub fn with_client(ai_client: AiClient) -> Self {
        Self { ai_client }
    }

    /// Начать генерацию стихотворения.
    ///
    /// Возвращает builder для настройки параметров.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::poetry::{Generator, PoetryStyle, Language, Meter};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let generator = Generator::new().await?;
    /// let poem = generator
    ///     .generate()
    ///     .theme("весна")
    ///     .style(PoetryStyle::Lyric)
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn generate(&self) -> PoemBuilder {
        PoemBuilder::new(&self.ai_client)
    }
}

/// Builder для создания стихотворения.
///
/// Позволяет настроить все параметры генерации.
pub struct PoemBuilder<'a> {
    /// AI клиент для генерации.
    ai_client: &'a AiClient,

    /// Тема стихотворения.
    theme: Option<String>,

    /// Стиль стихотворения.
    style: PoetryStyle,

    /// Язык стихотворения.
    language: Language,

    /// Стихотворный размер.
    meter: Meter,

    /// Количество строк (опционально).
    lines: Option<u32>,

    /// Название стихотворения (опционально).
    title: Option<String>,
}

impl<'a> PoemBuilder<'a> {
    /// Создать новый builder.
    fn new(ai_client: &'a AiClient) -> Self {
        Self {
            ai_client,
            theme: None,
            style: PoetryStyle::default(),
            language: Language::Russian,
            meter: Meter::default(),
            lines: None,
            title: None,
        }
    }

    /// Установить тему стихотворения.
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::poetry::Generator;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let generator = Generator::new().await?;
    /// let poem = generator
    ///     .generate()
    ///     .theme("любовь и разлука")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.theme = Some(theme.into());
        self
    }

    /// Установить стиль стихотворения.
    pub fn style(mut self, style: PoetryStyle) -> Self {
        self.style = style;
        self
    }

    /// Установить язык стихотворения.
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Установить стихотворный размер.
    pub fn meter(mut self, meter: Meter) -> Self {
        self.meter = meter;
        self
    }

    /// Установить количество строк.
    pub fn lines(mut self, lines: u32) -> Self {
        self.lines = Some(lines);
        self
    }

    /// Установить название стихотворения.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Сгенерировать стихотворение.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// - Тема не указана
    /// - Не удалось сгенерировать текст через AI
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use academia_lux::poetry::{Generator, PoetryStyle};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # let generator = Generator::new().await?;
    /// let poem = generator
    ///     .generate()
    ///     .theme("осень")
    ///     .style(PoetryStyle::Lyric)
    ///     .await?;
    ///
    /// println!("{}", poem.content);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn await(self) -> Result<Poem> {
        // Проверка темы
        let theme = self.theme.ok_or_else(|| {
            Error::PoetryGeneration("Theme is required for poem generation".to_string())
        })?;

        debug!(
            "Generating poem: theme='{}', style={:?}, language={:?}, meter={:?}",
            theme, self.style, self.language, self.meter
        );

        // Создание промпта
        let system_prompt = PoetryPrompts::system_prompt();
        let user_prompt = PoetryPrompts::generate_poem(
            &theme,
            self.style,
            self.language,
            self.meter,
            self.lines,
        );

        // Генерация через AI
        let content = self
            .ai_client
            .generate_with_system(system_prompt, &user_prompt)
            .await
            .map_err(|e| Error::PoetryGeneration(format!("AI generation failed: {}", e)))?;

        // Очистка текста (удаление лишних пробелов и переносов)
        let content = content.trim().to_string();

        // Генерация названия, если не указано
        let title = if let Some(title) = self.title {
            title
        } else {
            self.generate_title(&theme, &content).await?
        };

        // Создание стихотворения
        let poem = Poem::new(title, content, self.language, self.style, self.meter);

        info!(
            "Poem generated successfully: {} lines, {} words",
            poem.line_count(),
            poem.word_count()
        );

        Ok(poem)
    }

    /// Сгенерировать название для стихотворения.
    async fn generate_title(&self, theme: &str, _content: &str) -> Result<String> {
        // Простая генерация названия на основе темы
        // TODO: Можно улучшить, используя AI для генерации более креативных названий

        let title = match self.language {
            Language::Russian => format!("Стихотворение о {}", theme),
            Language::Greek => format!("Ποίημα για {}", theme),
            Language::Bulgarian => format!("Стихотворение за {}", theme),
            Language::English => format!("Poem about {}", theme),
        };

        Ok(title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poem_builder() {
        // Тест создания builder (без async)
        // Полные тесты требуют реального AI клиента
    }
}
