# Архитектура plantuml-rs

## Обзор

plantuml-rs — это модульная библиотека для рендеринга UML диаграмм. Архитектура разделена на независимые crates, каждый из которых отвечает за свою область.

---

## Диаграмма компонентов

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              plantuml-core                                   │
│                        (Публичный API / Фасад)                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  render(source: &str) -> Result<String, Error>                              │
│  render_to_png(source: &str) -> Result<Vec<u8>, Error>                      │
│  parse(source: &str) -> Result<Diagram, Error>                              │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
          ┌──────────────────────────┼──────────────────────────┐
          │                          │                          │
          ▼                          ▼                          ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   plantuml-     │     │   plantuml-     │     │   plantuml-     │
│  preprocessor   │────▶│     parser      │────▶│      ast        │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ • !include      │     │ • Lexer (logos) │     │ • Diagram enum  │
│ • !define       │     │ • Parser (pest) │     │ • Sequence AST  │
│ • !if/!else     │     │ • Грамматики    │     │ • Class AST     │
│ • !function     │     │                 │     │ • Activity AST  │
│ • %builtins     │     │                 │     │ • State AST     │
└─────────────────┘     └─────────────────┘     │ • ...           │
                                                └────────┬────────┘
                                                         │
                                                         ▼
                                               ┌─────────────────┐
                                               │   plantuml-     │
                                               │     model       │
                                               ├─────────────────┤
                                               │ • Типизированные│
                                               │   модели        │
                                               │ • Валидация     │
                                               │ • Трансформации │
                                               └────────┬────────┘
                                                        │
          ┌─────────────────────────────────────────────┼───────────────┐
          │                                             │               │
          ▼                                             ▼               ▼
┌─────────────────┐                        ┌─────────────────┐  ┌─────────────┐
│   plantuml-     │                        │   plantuml-     │  │  plantuml-  │
│     layout      │                        │     themes      │  │   stdlib    │
├─────────────────┤                        ├─────────────────┤  ├─────────────┤
│ • SequenceLayout│                        │ • Темы          │  │ • AWS icons │
│ • Sugiyama      │                        │ • skinparam     │  │ • Azure     │
│ • Flowchart     │                        │ • Стили         │  │ • K8s       │
│ • Tree          │                        │                 │  │ • C4        │
│ • Grid          │                        │                 │  │ • ...       │
└────────┬────────┘                        └────────┬────────┘  └──────┬──────┘
         │                                          │                  │
         └──────────────────────┬───────────────────┴──────────────────┘
                                │
                                ▼
                      ┌─────────────────┐
                      │   plantuml-     │
                      │    renderer     │
                      ├─────────────────┤
                      │ • SVG (svg)     │
                      │ • PNG (resvg)   │
                      │ • ASCII         │
                      │ • Shapes        │
                      │ • Text/Fonts    │
                      └────────┬────────┘
                               │
                               ▼
                      ┌─────────────────┐
                      │   plantuml-     │
                      │      wasm       │
                      ├─────────────────┤
                      │ • wasm-bindgen  │
                      │ • JS API        │
                      │ • NPM package   │
                      └─────────────────┘
```

---

## Описание crates

### plantuml-core

**Назначение**: Публичный API и фасад библиотеки.

**Зависимости**: Все остальные crates.

**Ключевые функции**:
```rust
pub fn render(source: &str) -> Result<String, Error>;
pub fn render_with_options(source: &str, options: RenderOptions) -> Result<String, Error>;
pub fn render_to_png(source: &str) -> Result<Vec<u8>, Error>;
pub fn parse(source: &str) -> Result<Diagram, Error>;
```

---

### plantuml-preprocessor

**Назначение**: Обработка директив препроцессора перед парсингом.

**Зависимости**: Минимальные (только std).

**Функциональность**:
- `!include <file>` / `!include_once`
- `!define` / `!undef`
- `!ifdef` / `!ifndef` / `!else` / `!endif`
- `!$variable = value`
- `!function` / `!procedure` / `!return`
- `!theme <name>`
- Builtin функции: `%date()`, `%version()`, `%filename()`, и 50+ других

**Пример**:
```rust
pub fn preprocess(source: &str, resolver: &dyn FileResolver) -> Result<String, PreprocessError>;
```

---

### plantuml-parser

**Назначение**: Лексический и синтаксический анализ.

**Зависимости**: `logos`, `pest`, `pest_derive`, `plantuml-ast`.

**Структура**:
```
plantuml-parser/
├── src/
│   ├── lib.rs
│   ├── lexer.rs           # Logos лексер
│   ├── grammars/          # Pest грамматики
│   │   ├── common.pest    # Общие правила
│   │   ├── sequence.pest
│   │   ├── class.pest
│   │   ├── activity.pest
│   │   └── ...
│   └── parsers/           # Парсеры для каждого типа
│       ├── mod.rs
│       ├── sequence.rs
│       ├── class.rs
│       └── ...
```

**Подход**: Двухфазный парсинг
1. **Лексер (logos)**: Быстрая токенизация
2. **Парсер (pest)**: PEG-грамматика для структуры

---

### plantuml-ast

**Назначение**: Типы AST для всех типов диаграмм.

**Зависимости**: Минимальные (`serde` для сериализации).

**Основная структура**:
```rust
pub enum Diagram {
    Sequence(SequenceDiagram),
    Class(ClassDiagram),
    Activity(ActivityDiagram),
    State(StateDiagram),
    Component(ComponentDiagram),
    Deployment(DeploymentDiagram),
    UseCase(UseCaseDiagram),
    Object(ObjectDiagram),
    Timing(TimingDiagram),
    Gantt(GanttDiagram),
    MindMap(MindMapDiagram),
    Wbs(WbsDiagram),
    Json(JsonDiagram),
    Yaml(YamlDiagram),
    Network(NetworkDiagram),
    Salt(SaltDiagram),
    Er(ErDiagram),
    Archimate(ArchimateDiagram),
}
```

---

### plantuml-model

**Назначение**: Типизированные модели для layout и рендеринга.

**Зависимости**: `plantuml-ast`.

**Задачи**:
- Преобразование AST в layout-модели
- Валидация семантики
- Разрешение ссылок

---

### plantuml-layout

**Назначение**: Алгоритмы автоматического размещения элементов.

**Зависимости**: `petgraph`, `plantuml-model`.

**Layout engines**:

| Engine | Диаграммы | Алгоритм |
|--------|-----------|----------|
| `SequenceLayout` | Sequence | Горизонтальное размещение участников, вертикальные lifelines |
| `HierarchicalLayout` | Class, Component, Deployment | Sugiyama algorithm |
| `FlowchartLayout` | Activity | Topological sort + lanes |
| `StateLayout` | State | Nested boxes |
| `TreeLayout` | MindMap, WBS | Tidy tree layout |
| `GridLayout` | Salt, Tables | Grid-based |
| `TimelineLayout` | Gantt, Timing | Timeline-based |

**Трейт**:
```rust
pub trait LayoutEngine {
    type Input;
    type Output;
    
    fn layout(&self, input: &Self::Input, config: &LayoutConfig) -> Self::Output;
}
```

---

### plantuml-renderer

**Назначение**: Генерация визуального вывода.

**Зависимости**: `svg`, `resvg`, `tiny-skia`, `fontdb`, `ab_glyph`.

**Рендереры**:
- **SvgRenderer**: Основной рендерер в SVG
- **PngRenderer**: SVG → PNG через resvg
- **AsciiRenderer**: Текстовый вывод (опционально)

**Трейт**:
```rust
pub trait Renderer {
    type Output;
    
    fn render(&self, layout: &LayoutResult, theme: &Theme) -> Self::Output;
}
```

---

### plantuml-themes

**Назначение**: Темы оформления и skinparam.

**Зависимости**: `serde`.

**Функциональность**:
- Встроенные темы (default, sketchy, etc.)
- Пользовательские темы
- skinparam параметры
- Цветовые схемы

---

### plantuml-stdlib

**Назначение**: Стандартная библиотека иконок и спрайтов.

**Зависимости**: Минимальные.

**Содержимое**:
- AWS Architecture Icons
- Azure Icons
- Kubernetes Icons
- C4 Model
- Material Design Icons
- И другие

---

### plantuml-wasm

**Назначение**: WASM биндинги для браузера.

**Зависимости**: `wasm-bindgen`, `plantuml-core`.

**API**:
```javascript
// JavaScript
import init, { render, parse } from 'plantuml-rs';

await init();
const svg = render('@startuml\nAlice -> Bob\n@enduml');
```

---

## Поток данных

```
Source Text
    │
    ▼
┌─────────────────┐
│   Preprocessor  │  Раскрытие !include, !define, etc.
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Lexer       │  Токенизация (logos)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Parser      │  Синтаксический анализ (pest)
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│      AST        │  Абстрактное синтаксическое дерево
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Model       │  Семантическая модель
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│     Layout      │  Вычисление позиций
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    Renderer     │  Генерация SVG/PNG
└────────┬────────┘
         │
         ▼
    Output (SVG/PNG)
```

---

## Принципы проектирования

### 1. Модульность
Каждый crate имеет чёткую ответственность и минимальные зависимости.

### 2. Расширяемость
Новые типы диаграмм добавляются через:
- Новый вариант в `Diagram` enum
- Новая грамматика в `plantuml-parser`
- Новый layout engine в `plantuml-layout`

### 3. WASM-совместимость
Все зависимости выбраны с учётом `wasm32-unknown-unknown` target.

### 4. Pure Rust
Никаких C/C++ зависимостей. Вся логика реализована на Rust.

### 5. Тестируемость
- Unit тесты в каждом модуле
- Integration тесты для full pipeline
- Visual regression тесты для рендеринга

---

## Зависимости между crates

```
plantuml-core
    ├── plantuml-parser
    │       ├── plantuml-ast
    │       └── plantuml-preprocessor
    ├── plantuml-model
    │       └── plantuml-ast
    ├── plantuml-layout
    │       └── plantuml-model
    ├── plantuml-renderer
    │       ├── plantuml-layout
    │       └── plantuml-themes
    ├── plantuml-themes
    └── plantuml-stdlib

plantuml-wasm
    └── plantuml-core
```

---

## Feature flags

```toml
[features]
default = ["svg"]
svg = []                    # SVG рендеринг (всегда включён)
png = ["resvg", "tiny-skia", "fontdb"]  # PNG рендеринг
wasm = []                   # WASM биндинги
all-diagrams = []           # Все типы диаграмм
sequence = []               # Только sequence diagrams
class = []                  # Только class diagrams
# ... и т.д.
```
