# Changelog

Все значимые изменения в проекте документируются в этом файле.

Формат основан на [Keep a Changelog](https://keepachangelog.com/ru/1.0.0/),
и проект придерживается [Семантического версионирования](https://semver.org/lang/ru/).

## [Unreleased]

### Добавлено
- (планируется)

---

## [0.2.0] - 2025-01-03

### Добавлено
- **Playground** — интерактивная веб-страница для тестирования диаграмм в браузере
- Поддержка 18 типов диаграмм (Sequence, Class, State, Activity, Component, UseCase, Object, Timing, Gantt, MindMap, WBS, JSON, YAML, ER, Network, Salt)
- 6 тем оформления (Default, Dark, Minimal, Sketchy, Cerulean)
- GitHub Pages публикация для онлайн-доступа к playground
- GitHub Actions CI workflow

### Исправлено
- **Детекция типа диаграммы**: sequence диаграммы с текстом "state" в сообщениях больше не определяются как State диаграммы
- **Перекрытие заголовка box**: заголовки participant box теперь корректно отображаются над карточками участников
- **Детерминированный вывод**: заменены HashMap/HashSet на IndexMap/IndexSet для стабильного SVG

### Изменено
- Обновлены URL репозитория на актуальные

---

## [0.1.0] - 2025-01-02

### Добавлено
- Инициализация проекта
- Структура директорий для workspace
- Документация проекта (README.md, AGENTS.md, PLAN.md)
- Файлы лицензий (MIT, Apache-2.0)
- Полная реализация парсеров для всех типов диаграмм
- Layout engines для всех типов диаграмм
- SVG и PNG рендеринг
- WASM биндинги
- Препроцессор PlantUML

---

## Предстоящие релизы

### [0.1.0] - Фаза 1 MVP (планируется)

**Цель**: Sequence Diagrams + Class Diagrams

#### Планируется добавить
- Препроцессор PlantUML (!include, !define, !if, etc.)
- Парсер Sequence Diagrams
- Парсер Class Diagrams
- Layout engine для Sequence Diagrams
- Layout engine для Class Diagrams (Sugiyama algorithm)
- SVG рендеринг
- Базовая поддержка skinparam

---

### [0.2.0] - Фаза 2 (планируется)

**Цель**: Activity + State + Component Diagrams

#### Планируется добавить
- Парсер Activity Diagrams (новый и legacy синтаксис)
- Парсер State Diagrams
- Парсер Component/Deployment Diagrams
- Flowchart layout engine
- Поддержка вложенных состояний

---

### [0.3.0] - Фаза 3 (планируется)

**Цель**: Остальные UML диаграммы

#### Планируется добавить
- Use Case Diagrams
- Object Diagrams
- Timing Diagrams
- Package Diagrams

---

### [0.4.0] - Фаза 4 (планируется)

**Цель**: Non-UML диаграммы

#### Планируется добавить
- Gantt Charts
- MindMap
- WBS
- JSON/YAML визуализация
- Network Diagrams (nwdiag)
- Salt (Wireframe)
- ER Diagrams
- Archimate

---

### [1.0.0] - Фаза 5 (планируется)

**Цель**: Production-ready релиз

#### Планируется добавить
- Полная WASM поддержка
- NPM package
- Оптимизация производительности
- Полная документация API
- Стандартная библиотека (AWS, Azure, Kubernetes icons)

---

## Формат записей

- `Добавлено` — для новой функциональности
- `Изменено` — для изменений в существующей функциональности
- `Устарело` — для функциональности, которая будет удалена
- `Удалено` — для удалённой функциональности
- `Исправлено` — для исправления ошибок
- `Безопасность` — для исправления уязвимостей
