# 🎭 Academia Lux - Hellenic Poetry AI Platform

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/status-in%20development-blue.svg)]()

**Academia Lux** - комплексная AI-платформа для создания стихов, обучения языкам, интерактивных театральных постановок и православного образования.

---

## 🎯 Основные возможности

### 1. Генерация стихов и песен
- ✅ Создание стихов на русском, греческом, болгарском языках
- ✅ Различные стихотворные размеры (ямб, хорей, дактиль)
- ✅ Интеграция с Suno App для создания музыки
- ✅ Генерация промптов для видеоклипов
- ✅ Рифмовка и ритмика

### 2. Обучение языкам
- ✅ Погружение через поэзию
- ✅ Интерактивные упражнения
- ✅ Перевод каждого слова в скобках
- ✅ Система уровней (A1-C2)
- ✅ Аудио произношение
- ✅ Отслеживание прогресса

### 3. Интерактивный театр
- ✅ Мюзиклы с изменяющимся сюжетом
- ✅ Голосование зрителей в реальном времени
- ✅ Генерация диалогов персонажей
- ✅ Музыкальное сопровождение
- ✅ Визуализация сцен

### 4. Православный AI-ассистент
- ✅ Работа с текстами Библии
- ✅ Святоотеческая литература
- ✅ Церковный календарь
- ✅ Апологетика (защита веры)
- ✅ Объяснение церковных терминов

### 5. Telegram бот (99 функций)
- ✅ Генератор персональных стихов
- ✅ AI-копирайтер
- ✅ Виртуальный художник
- ✅ Языковой тренажёр
- ✅ Персональный репетитор
- ✅ И многое другое...

### 6. Интеграция с Яндекс Алисой
- ✅ Голосовое управление
- ✅ Объяснение терминов
- ✅ Перевод богослужения
- ✅ Цитата дня

### 7. Система субтитров для служб
- ✅ HDMI подключение
- ✅ Тачскрин интерфейс
- ✅ Перевод сложных слов
- ✅ Исторические пояснения

---

## 🚀 Быстрый старт

### Установка

```bash
# Клонирование репозитория
git clone https://github.com/academia-lux/academia-lux.git
cd academia-lux

# Сборка проекта
cargo build --release

# Запуск
cargo run --release
```

### Конфигурация

Скопируйте `config/default.yaml` и настройте API ключи:

```yaml
ai:
  openai_api_key: "your-openai-key"
  anthropic_api_key: "your-anthropic-key"
  deepl_api_key: "your-deepl-key"
  suno_api_key: "your-suno-key"

telegram:
  bot_token: "your-telegram-bot-token"

alice:
  skill_id: "your-alice-skill-id"
  oauth_token: "your-alice-oauth-token"
```

Или используйте переменные окружения:

```bash
export OPENAI_API_KEY="your-key"
export TELEGRAM_BOT_TOKEN="your-token"
```

---

## 📖 Примеры использования

### Генерация стихотворения

```rust
use academia_lux::poetry::{Generator, PoetryStyle, Language, Meter};

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

### Урок греческого языка

```rust
use academia_lux::learning::{LanguageTutor, LanguageLevel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tutor = LanguageTutor::new().await?;

    let lesson = tutor
        .create_lesson()
        .level(LanguageLevel::A1)
        .topic("приветствия")
        .await?;

    lesson.teach().await?;
    Ok(())
}
```

### Интерактивная постановка

```rust
use academia_lux::theater::{InteractivePlay, Genre};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut play = InteractivePlay::new()
        .title("Одиссея: Новое путешествие")
        .genre(Genre::Fantasy)
        .await?;

    play.start().await?;
    play.vote("option_a", 150).await?;
    play.next_scene().await?;

    Ok(())
}
```

### Telegram бот

```bash
# Запуск бота
cargo run --bin telegram-bot

# Команды бота:
# /start - Начало работы
# /poem <тема> - Генерация стихотворения
# /lesson - Урок греческого языка
# /play - Интерактивная постановка
# /quote - Цитата дня
# /explain <термин> - Объяснение термина
```

---

## 🏗️ Архитектура

```
academia-lux/
├── src/
│   ├── lib.rs                      # Основной модуль
│   ├── main.rs                     # CLI приложение
│   ├── error.rs                    # Обработка ошибок
│   ├── config.rs                   # Конфигурация
│   │
│   ├── poetry/                     # Генерация стихов
│   │   ├── mod.rs
│   │   ├── generator.rs
│   │   ├── rhyme.rs
│   │   ├── meter.rs
│   │   ├── styles.rs
│   │   └── suno_integration.rs
│   │
│   ├── learning/                   # Обучение языкам
│   │   ├── mod.rs
│   │   ├── language_tutor.rs
│   │   ├── vocabulary.rs
│   │   ├── grammar.rs
│   │   ├── exercises.rs
│   │   ├── progress.rs
│   │   └── deepl_integration.rs
│   │
│   ├── theater/                    # Интерактивный театр
│   │   ├── mod.rs
│   │   ├── interactive_play.rs
│   │   ├── characters.rs
│   │   ├── plot.rs
│   │   ├── voting.rs
│   │   └── scenes.rs
│   │
│   ├── orthodox/                   # Православный модуль
│   │   ├── mod.rs
│   │   ├── bible.rs
│   │   ├── saints.rs
│   │   ├── calendar.rs
│   │   ├── prayers.rs
│   │   ├── liturgy.rs
│   │   └── apologetics.rs
│   │
│   ├── ai/                         # AI интеграция
│   │   ├── mod.rs
│   │   ├── openai_client.rs
│   │   ├── claude_client.rs
│   │   ├── prompts.rs
│   │   └── models.rs
│   │
│   ├── bot/                        # Боты
│   │   ├── mod.rs
│   │   ├── telegram.rs
│   │   ├── alice.rs
│   │   ├── commands.rs
│   │   └── handlers.rs
│   │
│   ├── game/                       # Игровая механика
│   │   ├── mod.rs
│   │   ├── achievements.rs
│   │   ├── levels.rs
│   │   └── leaderboard.rs
│   │
│   └── utils/                      # Утилиты
│       ├── mod.rs
│       ├── text_processing.rs
│       ├── audio.rs
│       └── database.rs
│
├── config/
│   ├── default.yaml
│   └── prompts/
│
├── data/
│   ├── greek_vocabulary.json
│   ├── russian_vocabulary.json
│   ├── bulgarian_vocabulary.json
│   ├── poetry_templates.json
│   ├── theater_plots.json
│   ├── bible_texts/
│   ├── saints_lives/
│   └── orthodox_texts/
│
├── examples/
│   ├── generate_poem.rs
│   ├── greek_lesson.rs
│   ├── interactive_play.rs
│   ├── telegram_bot.rs
│   └── orthodox_assistant.rs
│
├── tests/
│   ├── poetry_tests.rs
│   ├── learning_tests.rs
│   └── theater_tests.rs
│
├── Cargo.toml
├── README.md
├── TZ-FULL.md
└── LICENSE
```

---

## 🔧 Технологический стек

### Backend
- **Rust** - производительность и безопасность
- **Tokio** - асинхронный runtime
- **Axum** - web framework
- **SQLite/PostgreSQL** - база данных

### AI Integration
- **OpenAI GPT-4** - генерация текстов
- **Anthropic Claude** - альтернативный AI
- **Suno API** - генерация музыки
- **DALL-E / Midjourney** - визуализация
- **DeepL Pro** - переводы
- **Whisper** - распознавание речи

### Боты
- **Telegram Bot API**
- **Yandex Alice API**

### Frontend (планируется)
- **React/Vue** - web интерфейс
- **Flutter** - мобильное приложение

---

## 📊 Функциональность

### 400+ уникальных функций

Полный список функций см. в [TZ-FULL.md](TZ-FULL.md) и [R.md](R.md).

Основные категории:
1. **Генерация контента** (15 функций)
2. **Обучение языкам** (20 функций)
3. **Интерактивный театр** (15 функций)
4. **Православный ассистент** (20 функций)
5. **Образование и развитие** (15 функций)
6. **Здоровье и семья** (10 функций)
7. **Миссия и служение** (9 функций)
8. **И многое другое...**

---

## 🎯 Roadmap

### MVP (Этап 1) - 2-4 недели
- [x] Базовая структура проекта
- [x] Конфигурация и обработка ошибок
- [ ] Telegram бот с базовыми командами
- [ ] Генерация стихов (OpenAI)
- [ ] Простой словарь греческого языка

### Этап 2 - 1-2 месяца
- [ ] Интеграция с Suno API
- [ ] Система уроков греческого
- [ ] Интерактивные упражнения
- [ ] Отслеживание прогресса
- [ ] Интеграция с Яндекс Алисой

### Этап 3 - 3-6 месяцев
- [ ] Интерактивные театральные постановки
- [ ] Православный AI-ассистент
- [ ] Работа с текстами Библии
- [ ] Система субтитров для служб
- [ ] Web и Mobile приложения

---

## 🤝 Вклад в проект

Мы приветствуем вклад сообщества! Пожалуйста:

1. Fork репозитория
2. Создайте feature branch (`git checkout -b feature/amazing-feature`)
3. Commit изменения (`git commit -m 'Add amazing feature'`)
4. Push в branch (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

### Правила кода

- Следуйте Rust conventions
- Документируйте все публичные API
- Пишите тесты (coverage > 80%)
- Комментарии согласно PEP 8 стилю

---

## 📄 Лицензия

MIT License - см. [LICENSE](LICENSE)

---

## 📞 Контакты

- **Email:** support@academia-lux.org
- **GitHub:** https://github.com/academia-lux
- **Telegram:** @academia_lux_bot
- **Discord:** https://discord.gg/academia-lux

---

## 🙏 Благодарности

- **OpenAI** - за GPT-4 API
- **Anthropic** - за Claude API
- **Suno** - за музыкальную генерацию
- **DeepL** - за качественные переводы
- **Rust Community** - за отличный язык программирования

---

## 📚 Документация

- [Полное ТЗ](TZ-FULL.md)
- [Детальное ТЗ из R.md](R.md)
- [API Documentation](https://docs.academia-lux.org)
- [User Guide](https://guide.academia-lux.org)

---

**Версия:** 0.1.0
**Статус:** В разработке
**Дата:** 2026-01-11

---

Made with ❤️ for Orthodox education and Greek language learning
