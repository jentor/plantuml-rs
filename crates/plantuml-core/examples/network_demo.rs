//! Демонстрация Network (nwdiag) диаграмм

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    let examples = vec![
        (
            "simple",
            r#"@startuml
nwdiag {
  network dmz {
    address = "210.x.x.x/24"
    web01 [address = "210.x.x.1"]
    web02 [address = "210.x.x.2"]
  }
}
@enduml"#,
        ),
        (
            "multi_network",
            r#"@startuml
nwdiag {
  network dmz {
    address = "210.x.x.x/24"
    web01 [address = "210.x.x.1"]
    web02 [address = "210.x.x.2"]
  }
  network internal {
    address = "172.x.x.x/24"
    web01 [address = "172.x.x.1"]
    web02 [address = "172.x.x.2"]
    db01 [address = "172.x.x.100"]
  }
}
@enduml"#,
        ),
        (
            "three_tier",
            r#"@startuml
nwdiag {
  network internet {
    address = "0.0.0.0/0"
    firewall01 [address = "public"]
  }
  network dmz {
    address = "10.0.1.0/24"
    firewall01 [address = "10.0.1.1"]
    web01 [address = "10.0.1.10"]
    web02 [address = "10.0.1.11"]
  }
  network internal {
    address = "10.0.2.0/24"
    web01 [address = "10.0.2.10"]
    web02 [address = "10.0.2.11"]
    app01 [address = "10.0.2.20"]
    app02 [address = "10.0.2.21"]
  }
  network database {
    address = "10.0.3.0/24"
    app01 [address = "10.0.3.20"]
    app02 [address = "10.0.3.21"]
    db01 [address = "10.0.3.100"]
    db02 [address = "10.0.3.101"]
  }
}
@enduml"#,
        ),
        (
            "device_types",
            r#"@startuml
nwdiag {
  network office {
    address = "192.168.1.0/24"
    router01 [address = "192.168.1.1", type = router]
    switch01 [address = "192.168.1.2", type = switch]
    firewall01 [address = "192.168.1.3", type = firewall]
    server01 [address = "192.168.1.10", type = server]
    db01 [address = "192.168.1.20", type = database]
  }
}
@enduml"#,
        ),
    ];

    // Создаём директорию для вывода
    let output_dir = Path::new("target/network_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    let options = RenderOptions::default();

    for (name, source) in examples {
        match render(source, &options) {
            Ok(svg) => {
                let output_path = output_dir.join(format!("{}.svg", name));
                fs::write(&output_path, &svg).expect("Не удалось записать файл");
                println!("✓ Сохранено: {}", output_path.display());
            }
            Err(e) => {
                eprintln!("✗ Ошибка для {}: {}", name, e);
            }
        }
    }

    println!("\nГотово! Проверьте файлы в target/network_examples");
}
