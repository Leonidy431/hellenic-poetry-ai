# 🎭 Техническое задание: Hellenic Poetry AI Agent

## 📋 Описание проекта

**Hellenic Poetry AI Agent** - интерактивная AI-система для создания стихов и обучения греческому языку через погружение в поэзию и театральные постановки.

---

## 🎯 Основные функции

### 1. Генерация стихов (Poetry Generation)
- Создание стихов на русском и греческом языках
- Поддержка различных стихотворных размеров (ямб, хорей, дактиль и т.д.)
- Генерация текстов для песен (как Suno AI)
- Адаптация под музыкальные жанры
- Рифмовка и ритмика

### 2. Обучение греческому языку (Greek Language Learning)
- Погружение через поэзию
- Интерактивные упражнения
- Перевод и объяснение слов
- Грамматические конструкции в контексте стихов
- Произношение и фонетика
- Прогресс обучения пользователя

### 3. Театральные постановки (Interactive Theater)
- Интерактивные мюзиклы
- Изменяющийся сюжет в зависимости от зрителей
- Голосование за развитие сюжета
- Генерация диалогов персонажей
- Музыкальное сопровождение
- Визуализация сцен

### 4. Игровая среда (Gamification)
- Система достижений
- Уровни владения языком
- Коллекция созданных стихов
- Рейтинг пользователей
- Челленджи и квесты

---

## 🏗️ Архитектура системы

### Модули

```
hellenic-poetry-ai/
├── src/
│   ├── lib.rs                      # Основной модуль библиотеки
│   ├── main.rs                     # CLI приложение
│   ├── error.rs                    # Обработка ошибок
│   ├── config.rs                   # Конфигурация
│   │
│   ├── poetry/                     # Модуль генерации стихов
│   │   ├── mod.rs
│   │   ├── generator.rs            # Генератор стихов
│   │   ├── rhyme.rs                # Рифмовка
│   │   ├── meter.rs                # Стихотворный размер
│   │   └── styles.rs               # Стили и жанры
│   │
│   ├── learning/                   # Модуль обучения
│   │   ├── mod.rs
│   │   ├── greek_tutor.rs          # Обучение греческому
│   │   ├── vocabulary.rs           # Словарь
│   │   ├── grammar.rs              # Грамматика
│   │   ├── exercises.rs            # Упражнения
│   │   └── progress.rs             # Прогресс пользователя
│   │
│   ├── theater/                    # Модуль театра
│   │   ├── mod.rs
│   │   ├── interactive_play.rs     # Интерактивная постановка
│   │   ├── characters.rs           # Персонажи
│   │   ├── plot.rs                 # Сюжет
│   │   ├── voting.rs               # Голосование зрителей
│   │   └── scenes.rs               # Сцены
│   │
│   ├── ai/                         # AI интеграция
│   │   ├── mod.rs
│   │   ├── client.rs               # API клиент
│   │   ├── prompts.rs              # Промпты для AI
│   │   └── models.rs               # Модели данных
│   │
│   ├── game/                       # Игровая механика
│   │   ├── mod.rs
│   │   ├── achievements.rs         # Достижения
│   │   ├── levels.rs               # Уровни
│   │   ├── challenges.rs           # Челленджи
│   │   └── leaderboard.rs          # Рейтинг
│   │
│   └── utils/                      # Утилиты
│       ├── mod.rs
│       ├── text_processing.rs      # Обработка текста
│       └── database.rs             # База данных
│
├── config/
│   ├── default.yaml                # Конфигурация по умолчанию
│   └── prompts/                    # Промпты для AI
│       ├── poetry_generation.txt
│       ├── greek_teaching.txt
│       └── theater_script.txt
│
├── examples/
│   ├── generate_poem.rs            # Пример генерации стихов
│   ├── greek_lesson.rs             # Пример урока греческого
│   └── interactive_play.rs         # Пример театральной постановки
│
├── tests/
│   ├── poetry_tests.rs
│   ├── learning_tests.rs
│   └── theater_tests.rs
│
├── data/
│   ├── greek_vocabulary.json       # Словарь греческого языка
│   ├── poetry_templates.json       # Шаблоны стихов
│   └── theater_plots.json          # Сюжеты для театра
│
├── Cargo.toml
├── README.md
└── LICENSE
```

---

## 🔧 Технологический стек

### Backend
- **Язык:** Rust (производительность, безопасность)
- **Async runtime:** Tokio
- **HTTP:** Axum / Actix-web
- **Database:** SQLite / PostgreSQL
- **AI API:** OpenAI / Anthropic Claude / Local LLM

### Зависимости
```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }
clap = { version = "4.4", features = ["derive"] }
uuid = { version = "1.6", features = ["v4"] }
chrono = "0.4"
```

---

## 📊 Основные сущности

### 1. Poem (Стихотворение)
```rust
pub struct Poem {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub language: Language,
    pub style: PoetryStyle,
    pub meter: Meter,
    pub author_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
```

### 2. GreekLesson (Урок греческого)
```rust
pub struct GreekLesson {
    pub id: Uuid,
    pub title: String,
    pub level: LanguageLevel,
    pub vocabulary: Vec<Word>,
    pub exercises: Vec<Exercise>,
    pub poem: Poem,
}
```

### 3. InteractivePlay (Интерактивная постановка)
```rust
pub struct InteractivePlay {
    pub id: Uuid,
    pub title: String,
    pub current_scene: usize,
    pub scenes: Vec<Scene>,
    pub characters: Vec<Character>,
    pub audience_votes: HashMap<String, u32>,
}
```

### 4. User (Пользователь)
```rust
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub greek_level: LanguageLevel,
    pub achievements: Vec<Achievement>,
    pub created_poems: Vec<Uuid>,
    pub progress: LearningProgress,
}
```

---

## 🎨 Функциональные требования

### Генерация стихов
1. Пользователь задает тему, стиль, язык
2. AI генерирует стихотворение
3. Возможность редактирования и регенерации
4. Сохранение в коллекцию пользователя
5. Экспорт в различные форматы (txt, pdf, audio)

### Обучение греческому
1. Адаптивный уровень сложности
2. Новые слова вводятся через стихи
3. Интерактивные упражнения на перевод
4. Произношение с аудио
5. Отслеживание прогресса

### Театральные постановки
1. Выбор жанра мюзикла (комедия, драма, фэнтези)
2. Генерация начального сюжета
3. Голосование зрителей за развитие событий
4. Динамическая генерация диалогов
5. Музыкальные номера (интеграция с генерацией музыки)

### Игровая механика
1. Система уровней (A1-C2 для греческого)
2. Достижения за создание стихов
3. Ежедневные челленджи
4. Рейтинг лучших поэтов
5. Социальные функции (шаринг, комментарии)

---

## 🔐 Нефункциональные требования

### Производительность
- Генерация стихотворения: < 5 секунд
- Ответ на упражнение: < 1 секунды
- Поддержка 1000+ одновременных пользователей

### Безопасность
- Аутентификация пользователей
- Защита API ключей
- Валидация пользовательского ввода
- Rate limiting для AI запросов

### Масштабируемость
- Горизонтальное масштабирование
- Кэширование частых запросов
- Асинхронная обработка

### Качество кода
- Следование PEP 8 (для Python) / Rust conventions
- Документация всех публичных API
- Unit тесты (coverage > 80%)
- Integration тесты

---

## 🚀 Этапы разработки

### Этап 1: MVP (Минимально жизнеспособный продукт)
- [x] Базовая генерация стихов на русском
- [x] Простой словарь греческого языка
- [x] CLI интерфейс
- [x] Интеграция с OpenAI API

### Этап 2: Обучение греческому
- [ ] Система уроков
- [ ] Интерактивные упражнения
- [ ] Отслеживание прогресса
- [ ] Аудио произношение

### Этап 3: Театральные постановки
- [ ] Генерация сюжетов
- [ ] Система голосования
- [ ] Персонажи и диалоги
- [ ] Визуализация сцен

### Этап 4: Игровая механика
- [ ] Достижения и уровни
- [ ] Рейтинг пользователей
- [ ] Социальные функции
- [ ] Мобильное приложение

---

## 📝 Примеры использования

### Пример 1: Генерация стихотворения
```rust
use hellenic_poetry_ai::poetry::Generator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let generator = Generator::new().await?;

    let poem = generator
        .generate()
        .theme("море и свобода")
        .style(PoetryStyle::Lyric)
        .language(Language::Russian)
        .meter(Meter::Iambic)
        .await?;

    println!("{}", poem.content);
    Ok(())
}
```

### Пример 2: Урок греческого
```rust
use hellenic_poetry_ai::learning::GreekTutor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tutor = GreekTutor::new().await?;

    let lesson = tutor
        .create_lesson()
        .level(LanguageLevel::A1)
        .topic("приветствия")
        .await?;

    lesson.teach().await?;
    Ok(())
}
```

### Пример 3: Интерактивная постановка
```rust
use hellenic_poetry_ai::theater::InteractivePlay;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut play = InteractivePlay::new()
        .title("Одиссея: Новое путешествие")
        .genre(Genre::Fantasy)
        .await?;

    play.start().await?;

    // Зрители голосуют
    play.vote("option_a", 150).await?;
    play.vote("option_b", 200).await?;

    // Продолжение сюжета на основе голосов
    play.next_scene().await?;

    Ok(())
}
```

---

## 🎯 Метрики успеха

### Технические метрики
- Время генерации стихотворения: < 5 сек
- Uptime: > 99.5%
- Test coverage: > 80%
- API response time: < 200ms

### Пользовательские метрики
- Количество созданных стихов: 10,000+
- Активных пользователей: 1,000+
- Средняя длительность сессии: > 15 минут
- Retention rate: > 40%

---

## 📚 Документация

### Для разработчиков
- API Reference
- Architecture Decision Records (ADR)
- Contributing Guidelines
- Code Style Guide

### Для пользователей
- User Guide
- Tutorial Videos
- FAQ
- Community Forum

---

## 🤝 Вклад в проект

Проект открыт для вклада сообщества:
1. Fork репозитория
2. Создайте feature branch
3. Commit изменения
4. Push в branch
5. Создайте Pull Request

---

## 📄 Лицензия

MIT License

---

## 📞 Контакты

- **Email:** support@hellenic-poetry.ai
- **GitHub:** https://github.com/hellenic-poetry-ai
- **Discord:** https://discord.gg/hellenic-poetry

---

**Версия ТЗ:** 1.0
**Дата:** 2026-01-11
**Статус:** В разработке
