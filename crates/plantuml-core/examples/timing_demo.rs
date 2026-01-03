//! Демонстрация рендеринга Timing Diagrams
//!
//! Запуск: cargo run -p plantuml-core --example timing_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/timing_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простая timing диаграмма
    let simple = r#"
@startuml
robust "Web Browser" as WB
concise "Server" as S

@0
WB is Idle
S is Idle

@100
WB is Running
S is Processing

@200
WB is Idle
S is Idle
@enduml
"#;

    // Пример 2: HTTP запрос/ответ
    let http_request = r#"
@startuml
title HTTP Request Lifecycle

robust "Client" as C
robust "Server" as S
robust "Database" as D

@0
C is Idle
S is Idle
D is Idle

@50
C is Requesting
S is Idle

@100
C is Waiting
S is Processing

@150
S is QueryDB
D is Querying

@200
D is Returning
S is Processing

@250
S is Responding
C is Receiving

@300
C is Idle
S is Idle
D is Idle
@enduml
"#;

    // Пример 3: Digital signals (clock и binary)
    let digital_signals = r#"
@startuml
clock CLK
binary "Data" as D
binary "Enable" as E

@0
CLK is high
D is 0
E is 0

@50
CLK is low
D is 1

@100
CLK is high
E is 1

@150
CLK is low
D is 0

@200
CLK is high

@250
CLK is low
E is 0
@enduml
"#;

    // Пример 4: Процессор Pipeline
    let processor_pipeline = r#"
@startuml
title CPU Pipeline Stages

concise "Instruction 1" as I1
concise "Instruction 2" as I2
concise "Instruction 3" as I3

@0
I1 is Fetch

@20
I1 is Decode
I2 is Fetch

@40
I1 is Execute
I2 is Decode
I3 is Fetch

@60
I1 is Memory
I2 is Execute
I3 is Decode

@80
I1 is WriteBack
I2 is Memory
I3 is Execute

@100
I2 is WriteBack
I3 is Memory

@120
I3 is WriteBack
@enduml
"#;

    // Пример 5: Сетевой протокол
    let network_protocol = r#"
@startuml
title TCP Handshake

robust "Client" as C
robust "Server" as S

@0
C is CLOSED
S is LISTEN

@50
C is SYN_SENT

@100
S is SYN_RCVD

@150
C is ESTABLISHED

@200
S is ESTABLISHED
@enduml
"#;

    // Пример 6: Светофор
    let traffic_light = r#"
@startuml
title Traffic Light Sequence

robust "North-South" as NS
robust "East-West" as EW

@0
NS is Red
EW is Green

@100
EW is Yellow

@120
EW is Red
NS is Green

@220
NS is Yellow

@240
NS is Red
EW is Green
@enduml
"#;

    // Рендерим все примеры
    let examples = [
        ("simple", simple),
        ("http_request", http_request),
        ("digital_signals", digital_signals),
        ("processor_pipeline", processor_pipeline),
        ("network_protocol", network_protocol),
        ("traffic_light", traffic_light),
    ];

    let options = RenderOptions::default();

    for (name, source) in examples.iter() {
        match render(source, &options) {
            Ok(svg) => {
                let path = output_dir.join(format!("{}.svg", name));
                fs::write(&path, &svg).expect("Не удалось записать файл");
                println!("✓ Сохранено: {}", path.display());
            }
            Err(e) => {
                eprintln!("✗ Ошибка в примере {}: {:?}", name, e);
            }
        }
    }

    println!("\nГотово! Проверьте файлы в {}", output_dir.display());
}
