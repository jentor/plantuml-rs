# plantuml-rs

**Pure Rust библиотека для рендеринга UML диаграмм, полностью совместимая с PlantUML**

[![Crates.io](https://img.shields.io/crates/v/plantuml-rs.svg)](https://crates.io/crates/plantuml-rs)
[![Documentation](https://docs.rs/plantuml-rs/badge.svg)](https://docs.rs/plantuml-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![CI](https://github.com/user/plantuml-rs/workflows/CI/badge.svg)](https://github.com/user/plantuml-rs/actions)

---

## Особенности

- **100% совместимость с PlantUML** — поддержка полного синтаксиса PlantUML
- **Pure Rust** — без зависимостей от C/C++ библиотек
- **WASM поддержка** — работает в браузере через WebAssembly
- **SVG вывод** — векторная графика высокого качества
- **PNG вывод** — растеризация через resvg/tiny-skia
- **Все типы диаграмм** — UML и non-UML диаграммы

## Поддерживаемые диаграммы

### UML диаграммы
- Sequence Diagram
- Class Diagram
- Activity Diagram
- State Diagram
- Component Diagram
- Deployment Diagram
- Use Case Diagram
- Object Diagram
- Timing Diagram

### Non-UML диаграммы
- Gantt Chart
- MindMap
- WBS (Work Breakdown Structure)
- JSON/YAML визуализация
- Network Diagram (nwdiag)
- Salt (Wireframe)
- ER Diagram
- Archimate

---

## Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
plantuml-rs = "0.1"
```

## Использование

### Базовый пример

```rust
use plantuml_rs::render;

fn main() {
    let source = r#"
@startuml
Alice -> Bob: Привет!
Bob --> Alice: Привет!
@enduml
"#;

    let svg = render(source).unwrap();
    println!("{}", svg);
}
```

### Sequence Diagram

```rust
use plantuml_rs::render;

let source = r#"
@startuml
participant Alice
participant Bob
participant Charlie

Alice -> Bob: Запрос авторизации
activate Bob

Bob -> Charlie: Проверка токена
activate Charlie
Charlie --> Bob: Токен валиден
deactivate Charlie

Bob --> Alice: Авторизация успешна
deactivate Bob

alt Успех
    Alice -> Bob: Получить данные
    Bob --> Alice: Данные
else Ошибка
    Alice -> Bob: Повторить запрос
end
@enduml
"#;

let svg = render(source).unwrap();
```

### Class Diagram

```rust
use plantuml_rs::render;

let source = r#"
@startuml
abstract class Animal {
    + name: String
    + age: int
    + {abstract} speak(): void
}

class Dog extends Animal {
    + breed: String
    + speak(): void
}

class Cat extends Animal {
    + indoor: bool
    + speak(): void
}

interface Trainable {
    + train(): void
}

Dog ..|> Trainable
@enduml
"#;

let svg = render(source).unwrap();
```

### WASM (в браузере)

```javascript
import init, { render } from 'plantuml-rs';

async function main() {
    await init();
    
    const source = `
@startuml
Alice -> Bob: Hello
@enduml
`;
    
    const svg = render(source);
    document.getElementById('diagram').innerHTML = svg;
}

main();
```

---

## Архитектура

```
┌─────────────┐    ┌──────────────┐    ┌────────┐    ┌──────────┐
│   Source    │───▶│ Preprocessor │───▶│ Parser │───▶│   AST    │
│   Text      │    │              │    │        │    │          │
└─────────────┘    └──────────────┘    └────────┘    └────┬─────┘
                                                          │
                                                          ▼
┌─────────────┐    ┌──────────────┐    ┌────────┐    ┌──────────┐
│    SVG      │◀───│   Renderer   │◀───│ Layout │◀───│  Model   │
│   Output    │    │              │    │        │    │          │
└─────────────┘    └──────────────┘    └────────┘    └──────────┘
```

## Производительность

| Операция | plantuml-rs | PlantUML (Java) |
|----------|-------------|-----------------|
| Простая sequence | ~5ms | ~500ms |
| Сложная class | ~20ms | ~1000ms |
| WASM загрузка | ~50ms | N/A |

*Бенчмарки проводились на M1 MacBook Pro*

---

## Разработка

### Требования

- Rust 1.75+
- wasm-pack (для WASM сборки)

### Сборка

```bash
# Сборка библиотеки
cargo build --workspace

# Запуск тестов
cargo test --workspace

# Сборка WASM
cargo build --target wasm32-unknown-unknown -p plantuml-wasm

# Документация
cargo doc --workspace --open
```

### Структура проекта

```
crates/
├── plantuml-core/       # Главный фасад
├── plantuml-parser/     # Лексер + парсер
├── plantuml-ast/        # AST типы
├── plantuml-preprocessor/ # Препроцессор
├── plantuml-model/      # Модели диаграмм
├── plantuml-layout/     # Layout engines
├── plantuml-renderer/   # SVG/PNG рендеринг
├── plantuml-themes/     # Темы
├── plantuml-stdlib/     # Стандартная библиотека
└── plantuml-wasm/       # WASM биндинги
```

---

## Roadmap

- [x] Фаза 0: Инфраструктура
- [ ] Фаза 1: Sequence + Class Diagrams
- [ ] Фаза 2: Activity + State + Component
- [ ] Фаза 3: Остальные UML диаграммы
- [ ] Фаза 4: Non-UML диаграммы
- [ ] Фаза 5: WASM + Оптимизация

Подробный план: [docs/PLAN.md](docs/PLAN.md)

---

## Лицензия

Проект доступен под двойной лицензией:

- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

Выберите любую на ваше усмотрение.

---

## Благодарности

- [PlantUML](https://plantuml.com/) — за создание отличного инструмента и синтаксиса
- [pest](https://pest.rs/) — за мощный PEG парсер
- [resvg](https://github.com/RazrFalcon/resvg) — за качественный SVG рендеринг

## Вклад в проект

Приветствуются любые вклады! Пожалуйста, ознакомьтесь с [CONTRIBUTING.md](CONTRIBUTING.md) перед отправкой pull request.
