//! Демонстрация рендеринга Deployment Diagrams
//!
//! Запуск: cargo run -p plantuml-core --example deployment_demo

use plantuml_core::{render, RenderOptions};
use std::fs;
use std::path::Path;

fn main() {
    // Создаём директорию для выходных файлов
    let output_dir = Path::new("target/deployment_examples");
    fs::create_dir_all(output_dir).expect("Не удалось создать директорию");

    // Пример 1: Простое развёртывание
    let simple = r#"
@startuml
node "Web Server" {
    [Apache]
}

node "App Server" {
    [Tomcat]
}

database "MySQL"

[Apache] --> [Tomcat]
[Tomcat] --> MySQL
@enduml
"#;

    // Пример 2: Микросервисная архитектура
    let microservices = r#"
@startuml
node "Load Balancer" {
    [nginx]
}

node "Application Cluster" {
    [Service A]
    [Service B]
    [Service C]
}

node "Data Layer" {
    database PostgreSQL
    database Redis
}

cloud "External Services" {
    [AWS S3]
    [SendGrid]
}

[nginx] --> [Service A]
[nginx] --> [Service B]
[nginx] --> [Service C]

[Service A] --> PostgreSQL
[Service B] --> Redis
[Service C] --> [AWS S3]
[Service B] --> [SendGrid]
@enduml
"#;

    // Пример 3: Kubernetes кластер
    let kubernetes = r#"
@startuml
node "Kubernetes Cluster" {
    node "Master Node" {
        [API Server]
        [Controller Manager]
        [Scheduler]
    }
    
    node "Worker Node 1" {
        [Pod: Frontend]
        [Pod: Backend]
    }
    
    node "Worker Node 2" {
        [Pod: Database]
        [Pod: Cache]
    }
}

cloud "Cloud Provider" {
    storage "Persistent Volume"
    [Load Balancer]
}

[Load Balancer] --> [Pod: Frontend]
[Pod: Frontend] --> [Pod: Backend]
[Pod: Backend] --> [Pod: Database]
[Pod: Backend] --> [Pod: Cache]
[Pod: Database] --> [Persistent Volume]
@enduml
"#;

    // Пример 4: Docker Compose
    let docker = r#"
@startuml
node "Docker Host" {
    node "docker-compose" {
        [web]
        [api]
        [worker]
        database db
        queue rabbitmq
    }
}

[web] --> [api] : HTTP
[api] --> db : SQL
[api] --> rabbitmq : AMQP
[worker] --> rabbitmq : consume
[worker] --> db : write
@enduml
"#;

    // Пример 5: Облачная инфраструктура
    let cloud_infra = r#"
@startuml
cloud "AWS" {
    node "VPC" {
        node "Public Subnet" {
            [ALB]
            [NAT Gateway]
        }
        
        node "Private Subnet" {
            [EC2: Web]
            [EC2: API]
        }
        
        node "Data Subnet" {
            database "RDS PostgreSQL"
            storage "ElastiCache Redis"
        }
    }
    
    storage "S3 Bucket"
    [CloudFront]
}

actor User

User --> [CloudFront]
[CloudFront] --> [ALB]
[ALB] --> [EC2: Web]
[EC2: Web] --> [EC2: API]
[EC2: API] --> [RDS PostgreSQL]
[EC2: API] --> [ElastiCache Redis]
[EC2: API] --> [S3 Bucket]
@enduml
"#;

    // Пример 6: Device deployment (IoT)
    let iot = r#"
@startuml
node "Data Center" {
    [MQTT Broker]
    database "Time Series DB"
    [Analytics Engine]
}

node "Edge Gateway" {
    [Edge Computing]
    [Protocol Converter]
}

node "Sensor Network" {
    [Temperature Sensor]
    [Humidity Sensor]
    [Motion Sensor]
}

[Temperature Sensor] --> [Protocol Converter]
[Humidity Sensor] --> [Protocol Converter]
[Motion Sensor] --> [Protocol Converter]
[Protocol Converter] --> [Edge Computing]
[Edge Computing] --> [MQTT Broker]
[MQTT Broker] --> [Time Series DB]
[Time Series DB] --> [Analytics Engine]
@enduml
"#;

    // Рендерим все примеры
    let examples = [
        ("simple", simple),
        ("microservices", microservices),
        ("kubernetes", kubernetes),
        ("docker", docker),
        ("cloud_infra", cloud_infra),
        ("iot", iot),
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
