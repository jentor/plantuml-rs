# AGENTS.md — Руководство для AI-агентов

## Проект: plantuml-rs

**Pure Rust библиотека для рендеринга UML диаграмм, полностью совместимая с PlantUML**

---

## КРИТИЧЕСКИ ВАЖНО

- Строго использовать **русский язык** для всех коммуникаций, документации и комментариев кода
- Весь код должен быть **Pure Rust** — никаких C/C++ зависимостей
- Код должен компилироваться для **WASM** (wasm32-unknown-unknown)
- **100% совместимость** с синтаксисом PlantUML

---

## 1. ТЕХНИЧЕСКИЙ СТЕК

### 1.1 Обязательные зависимости

```toml
[workspace.dependencies]
# Парсинг
logos = "0.14"           # Лексер (WASM-совместим)
pest = "2.7"             # PEG парсер (WASM-совместим)
pest_derive = "2.7"      # Макросы для pest

# Структуры данных
petgraph = { version = "0.6", default-features = false }  # Графы
indexmap = "2.0"         # Упорядоченные HashMap
smallvec = "1.13"        # Оптимизация мелких векторов

# Рендеринг
svg = "0.17"             # SVG генерация
resvg = { version = "0.42", default-features = false }    # SVG → PNG
tiny-skia = "0.11"       # CPU растеризатор
fontdb = { version = "0.21", default-features = false }   # Шрифты
ab_glyph = "0.2"         # Растеризация глифов

# Утилиты
thiserror = "1.0"        # Типизированные ошибки
serde = { version = "1.0", features = ["derive"] }        # Сериализация

# WASM
wasm-bindgen = "0.2"     # JS биндинги
```

### 1.2 Требования к Rust

- **Минимальная версия**: Rust 1.75+
- **Edition**: 2021
- **Targets**: 
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
  - `wasm32-unknown-unknown` (обязательно!)

### 1.3 WASM-совместимость

При добавлении зависимостей проверяй:
- Поддерживает ли crate `wasm32-unknown-unknown`
- Нет ли системных вызовов (filesystem, network)
- Для шрифтов: отключать `system_fonts` feature

```toml
# Пример WASM-совместимой конфигурации
[target.'cfg(target_arch = "wasm32")'.dependencies]
fontdb = { version = "0.21", default-features = false }
```

---

## 2. СТРУКТУРА ПРОЕКТА

```
plantuml-rs/
├── Cargo.toml                    # Workspace root
├── AGENTS.md                     # ЭТО ТЕКУЩИЙ ФАЙЛ
├── README.md
├── LICENSE-MIT
├── LICENSE-APACHE
├── CHANGELOG.md
├── CONTRIBUTING.md
│
├── crates/
│   ├── plantuml-core/           # Главный фасад — публичный API
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   │
│   ├── plantuml-ast/            # AST типы для всех диаграмм
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── sequence.rs      # AST для sequence diagrams
│   │       ├── class.rs         # AST для class diagrams
│   │       └── ...
│   │
│   ├── plantuml-parser/         # Лексер + Парсер
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── lexer.rs         # Logos лексер
│   │       ├── grammars/        # Pest грамматики
│   │       │   ├── sequence.pest
│   │       │   ├── class.pest
│   │       │   └── ...
│   │       └── parsers/         # Парсеры для каждого типа
│   │           ├── sequence.rs
│   │           ├── class.rs
│   │           └── ...
│   │
│   ├── plantuml-preprocessor/   # Препроцессор (!include, !define)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── directives.rs    # Парсинг директив
│   │       ├── variables.rs     # Переменные и подстановка
│   │       ├── functions.rs     # !function, !procedure
│   │       └── builtins.rs      # %date(), %version(), etc.
│   │
│   ├── plantuml-model/          # Типизированные модели
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── ...
│   │
│   ├── plantuml-layout/         # Layout engines
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── traits.rs        # trait LayoutEngine
│   │       ├── sequence/        # Layout для sequence diagrams
│   │       ├── hierarchical/    # Sugiyama algorithm
│   │       ├── flowchart/       # Activity diagrams
│   │       └── tree/            # MindMap, WBS
│   │
│   ├── plantuml-renderer/       # SVG/PNG рендеринг
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── svg.rs           # SVG генерация
│   │       ├── png.rs           # PNG через resvg
│   │       └── shapes/          # Примитивы рисования
│   │
│   ├── plantuml-themes/         # Темы и skinparam
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   ├── plantuml-stdlib/         # Стандартная библиотека
│   │   ├── Cargo.toml
│   │   └── src/
│   │
│   └── plantuml-wasm/           # WASM биндинги
│       ├── Cargo.toml
│       └── src/lib.rs
│
├── tests/
│   ├── visual/                  # Визуальные тесты (insta snapshots)
│   ├── compatibility/           # Тесты совместимости с PlantUML
│   └── fixtures/                # Тестовые .puml файлы
│       ├── sequence/
│       ├── class/
│       └── ...
│
├── examples/                    # Примеры использования
│
├── benches/                     # Бенчмарки
│
├── docs/                        # Документация
│   ├── PLAN.md                  # Детальный план разработки
│   ├── ARCHITECTURE.md          # Архитектура системы
│   └── ...
│
└── .github/
    └── workflows/
        ├── ci.yml               # CI pipeline
        └── release.yml          # Релизы
```

---

## 3. ТЕКУЩАЯ ФАЗА РАЗРАБОТКИ

### Фаза 0: Инфраструктура (ТЕКУЩАЯ)

**Статус**: В процессе

**Задачи**:
- [x] Создать структуру папок
- [x] Документация (PLAN.md, AGENTS.md)
- [ ] Инициализировать Cargo workspace
- [ ] Настроить CI/CD (GitHub Actions)
- [ ] Создать базовые типы в `plantuml-ast`
- [ ] Создать трейты в `plantuml-layout` и `plantuml-renderer`
- [ ] Настроить тестирование (insta для snapshots)

**Следующая фаза**: Фаза 1 — Sequence + Class Diagrams

---

## 4. ПРАВИЛА РАЗРАБОТКИ

### 4.1 Код

```rust
// Использовать типизированные ошибки
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("неожиданный токен: {0}")]
    UnexpectedToken(String),
    
    #[error("синтаксическая ошибка в строке {line}: {message}")]
    SyntaxError { line: usize, message: String },
}

// Документировать публичные API на русском
/// Парсит PlantUML исходный код и возвращает AST диаграммы.
///
/// # Аргументы
/// * `source` - исходный код PlantUML
///
/// # Возвращает
/// * `Ok(Diagram)` - распарсенная диаграмма
/// * `Err(ParseError)` - ошибка парсинга
pub fn parse(source: &str) -> Result<Diagram, ParseError> {
    // ...
}
```

### 4.2 Тестирование

```rust
// Unit тесты в том же файле
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_sequence() {
        let source = "@startuml\nAlice -> Bob: Hello\n@enduml";
        let result = parse(source);
        assert!(result.is_ok());
    }
}

// Визуальные тесты с insta
#[test]
fn test_sequence_render() {
    let svg = render("@startuml\nAlice -> Bob\n@enduml").unwrap();
    insta::assert_snapshot!(svg);
}
```

### 4.3 Комментарии и документация

- **Комментарии кода**: на русском языке
- **Doc comments (///)**: на русском языке
- **README, CHANGELOG**: на русском языке
- **Сообщения коммитов**: на русском языке

### 4.4 Git workflow

```bash
# Формат коммитов
git commit -m "feat(parser): добавлен парсинг sequence diagrams"
git commit -m "fix(layout): исправлен расчёт позиции lifeline"
git commit -m "docs: обновлена документация API"
git commit -m "test: добавлены тесты для class diagrams"
git commit -m "refactor(renderer): оптимизация SVG генерации"
```

---

## 5. АЛГОРИТМЫ LAYOUT

### 5.1 Sequence Diagrams

**Алгоритм**:
1. Расположить участников горизонтально с равными интервалами
2. Рассчитать lifelines (вертикальные линии)
3. Для каждого сообщения назначить Y-координату
4. Обработать фрагменты (alt, opt, loop) — вложенные прямоугольники
5. Рассчитать активации (стек)

**Ключевые параметры**:
- `participant_spacing`: 80px
- `message_spacing`: 30px
- `activation_width`: 10px
- `fragment_padding`: 10px

### 5.2 Class Diagrams (Sugiyama Algorithm)

**Шаги**:
1. **Cycle Removal**: удаление циклов (обратные рёбра помечаются)
2. **Layer Assignment**: распределение узлов по слоям (longest path)
3. **Dummy Nodes**: добавление фиктивных узлов для длинных рёбер
4. **Crossing Minimization**: минимизация пересечений (barycenter heuristic)
5. **Coordinate Assignment**: назначение X,Y координат (Brandes-Köpf)
6. **Edge Routing**: ортогональная маршрутизация рёбер

**Референсы**:
- Sugiyama, K., Tagawa, S., & Toda, M. (1981). Methods for visual understanding of hierarchical system structures.
- Brandes, U., & Köpf, B. (2001). Fast and simple horizontal coordinate assignment.

### 5.3 Activity Diagrams (Flowchart)

**Алгоритм**:
1. Построить граф потока управления
2. Применить топологическую сортировку
3. Разместить узлы по уровням
4. Обработать swim lanes (горизонтальное разделение)
5. Маршрутизация рёбер с минимизацией пересечений

---

## 6. ТИПЫ ДИАГРАММ

### Приоритет реализации

| Приоритет | Тип | Сложность |
|-----------|-----|-----------|
| 1 | Sequence Diagram | Высокая |
| 2 | Class Diagram | Высокая |
| 3 | Activity Diagram | Средняя |
| 4 | State Diagram | Средняя |
| 5 | Component Diagram | Средняя |
| 6 | Use Case Diagram | Низкая |
| 7 | Object Diagram | Низкая |
| 8 | Deployment Diagram | Средняя |
| 9 | Timing Diagram | Высокая |
| 10+ | Non-UML (Gantt, MindMap, etc.) | Разная |

---

## 7. REVERSE ENGINEERING PlantUML

### 7.1 Ресурсы

- **Исходный код**: https://github.com/plantuml/plantuml
- **Ключевые пакеты**:
  - `net.sourceforge.plantuml.sequencediagram` — sequence diagrams
  - `net.sourceforge.plantuml.classdiagram` — class diagrams
  - `net.sourceforge.plantuml.svek` — layout engine
  - `net.sourceforge.plantuml.graphic` — рендеринг

### 7.2 Подход

1. Изучить структуру AST в Java коде
2. Проанализировать алгоритмы layout (особенно Svek)
3. Собрать тестовые примеры для каждой конструкции
4. Сравнивать вывод нашей библиотеки с оригиналом

---

## 8. ЧЕКЛИСТ ДЛЯ КАЖДОГО ТИПА ДИАГРАММ

При реализации нового типа диаграммы:

- [ ] Изучить синтаксис в документации PlantUML
- [ ] Собрать тестовые примеры (5-10 базовых, 5-10 сложных)
- [ ] Создать pest грамматику в `plantuml-parser/src/grammars/`
- [ ] Создать AST типы в `plantuml-ast/src/`
- [ ] Реализовать парсер в `plantuml-parser/src/parsers/`
- [ ] Реализовать layout в `plantuml-layout/src/`
- [ ] Добавить рендеринг в `plantuml-renderer/src/`
- [ ] Написать unit тесты
- [ ] Написать visual regression тесты
- [ ] Проверить совместимость с PlantUML
- [ ] Обновить документацию

---

## 9. ПОЛЕЗНЫЕ КОМАНДЫ

```bash
# Сборка
cargo build --workspace

# Тесты
cargo test --workspace

# Проверка WASM
cargo build --target wasm32-unknown-unknown -p plantuml-wasm

# Бенчмарки
cargo bench

# Документация
cargo doc --workspace --open

# Линтер
cargo clippy --workspace -- -D warnings

# Форматирование
cargo fmt --all
```

---

## 10. КОНТАКТЫ И РЕСУРСЫ

- **PlantUML официальный сайт**: https://plantuml.com/
- **PlantUML Language Reference**: https://plantuml.com/guide
- **pest Book**: https://pest.rs/book/
- **logos Documentation**: https://logos.maciej.codes/
- **petgraph Documentation**: https://docs.rs/petgraph/
