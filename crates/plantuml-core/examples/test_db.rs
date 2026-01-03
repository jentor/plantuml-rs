fn main() {
    let source = r#"@startuml
participant "Пользователь" as User
participant "Сервер" as Server
database "База данных" as DB

User -> Server: GET /api/users
Server -> DB: SELECT * FROM users
DB --> Server: users[]
Server --> User: 200 OK
@enduml"#;

    println!("=== Тест полного рендеринга ===");
    match plantuml_core::render(source, &plantuml_core::RenderOptions::default()) {
        Ok(svg) => {
            println!("Успех! SVG: {} байт", svg.len());
            std::fs::write("/tmp/test_db.svg", &svg).unwrap();
            println!("Сохранено в /tmp/test_db.svg");
        }
        Err(e) => println!("Ошибка: {}", e),
    }
}
