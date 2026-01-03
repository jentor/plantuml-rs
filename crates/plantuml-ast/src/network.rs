//! AST типы для Network (nwdiag) диаграмм
//!
//! Network диаграммы визуализируют сетевую топологию.
//!
//! Синтаксис PlantUML:
//! ```text
//! @startuml
//! nwdiag {
//!   network dmz {
//!     address = "210.x.x.x/24"
//!     web01 [address = "210.x.x.1"]
//!     web02 [address = "210.x.x.2"]
//!   }
//!   network internal {
//!     address = "172.x.x.x/24"
//!     web01 [address = "172.x.x.1"]
//!     db01
//!   }
//! }
//! @enduml
//! ```

use serde::{Deserialize, Serialize};

use crate::common::{Color, DiagramMetadata, Identifier};

/// Network диаграмма
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkDiagram {
    /// Метаданные диаграммы
    pub metadata: DiagramMetadata,
    /// Сети
    pub networks: Vec<Network>,
    /// Серверы/устройства (могут быть вне сетей)
    pub servers: Vec<Server>,
    /// Группы серверов
    pub groups: Vec<ServerGroup>,
}

impl NetworkDiagram {
    /// Создаёт новую пустую Network диаграмму
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавляет сеть
    pub fn add_network(&mut self, network: Network) {
        self.networks.push(network);
    }

    /// Добавляет сервер
    pub fn add_server(&mut self, server: Server) {
        self.servers.push(server);
    }

    /// Находит сервер по имени
    pub fn find_server(&self, name: &str) -> Option<&Server> {
        // Ищем в глобальных серверах
        if let Some(s) = self.servers.iter().find(|s| s.id.name == name) {
            return Some(s);
        }
        // Ищем в сетях
        for network in &self.networks {
            if let Some(s) = network.members.iter().find(|s| s.id.name == name) {
                return Some(s);
            }
        }
        None
    }

    /// Возвращает все уникальные серверы
    pub fn all_servers(&self) -> Vec<&Server> {
        let mut servers: Vec<&Server> = self.servers.iter().collect();
        for network in &self.networks {
            for member in &network.members {
                if !servers.iter().any(|s| s.id.name == member.id.name) {
                    servers.push(member);
                }
            }
        }
        servers
    }
}

/// Сеть
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    /// Идентификатор сети
    pub id: Identifier,
    /// Адрес сети (например "192.168.1.0/24")
    pub address: Option<String>,
    /// Цвет сети
    pub color: Option<Color>,
    /// Члены сети (серверы)
    pub members: Vec<Server>,
    /// Описание
    pub description: Option<String>,
}

impl Network {
    /// Создаёт новую сеть
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Identifier::new(name),
            address: None,
            color: None,
            members: Vec::new(),
            description: None,
        }
    }

    /// Устанавливает адрес сети
    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Добавляет сервер в сеть
    pub fn add_member(&mut self, server: Server) {
        self.members.push(server);
    }
}

/// Сервер/устройство
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Идентификатор сервера
    pub id: Identifier,
    /// IP адрес в сети
    pub address: Option<String>,
    /// Описание
    pub description: Option<String>,
    /// Тип устройства
    pub device_type: DeviceType,
    /// Цвет
    pub color: Option<Color>,
}

impl Server {
    /// Создаёт новый сервер
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Identifier::new(name),
            address: None,
            description: None,
            device_type: DeviceType::Server,
            color: None,
        }
    }

    /// Устанавливает адрес
    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Устанавливает описание
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Устанавливает тип устройства
    pub fn with_type(mut self, device_type: DeviceType) -> Self {
        self.device_type = device_type;
        self
    }
}

/// Группа серверов
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerGroup {
    /// Имя группы
    pub name: String,
    /// Цвет группы
    pub color: Option<Color>,
    /// Описание
    pub description: Option<String>,
    /// Серверы в группе
    pub servers: Vec<String>,
}

impl ServerGroup {
    /// Создаёт новую группу
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            color: None,
            description: None,
            servers: Vec::new(),
        }
    }
}

/// Тип устройства
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DeviceType {
    /// Сервер (по умолчанию)
    #[default]
    Server,
    /// Рабочая станция
    Workstation,
    /// Роутер
    Router,
    /// Коммутатор
    Switch,
    /// Firewall
    Firewall,
    /// База данных
    Database,
    /// Облако
    Cloud,
    /// Принтер
    Printer,
    /// Мобильное устройство
    Mobile,
}

impl DeviceType {
    /// Парсит тип из строки
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "server" => Some(Self::Server),
            "workstation" | "pc" | "desktop" => Some(Self::Workstation),
            "router" => Some(Self::Router),
            "switch" => Some(Self::Switch),
            "firewall" => Some(Self::Firewall),
            "database" | "db" => Some(Self::Database),
            "cloud" => Some(Self::Cloud),
            "printer" => Some(Self::Printer),
            "mobile" | "phone" => Some(Self::Mobile),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_network() {
        let mut network = Network::new("dmz").with_address("192.168.1.0/24");
        network.add_member(Server::new("web01").with_address("192.168.1.1"));
        network.add_member(Server::new("web02").with_address("192.168.1.2"));

        assert_eq!(network.id.name, "dmz");
        assert_eq!(network.members.len(), 2);
    }

    #[test]
    fn test_network_diagram() {
        let mut diagram = NetworkDiagram::new();

        let mut dmz = Network::new("dmz");
        dmz.add_member(Server::new("web01"));
        diagram.add_network(dmz);

        let mut internal = Network::new("internal");
        internal.add_member(Server::new("db01"));
        diagram.add_network(internal);

        assert_eq!(diagram.networks.len(), 2);
        assert_eq!(diagram.all_servers().len(), 2);
    }

    #[test]
    fn test_device_type_parse() {
        assert_eq!(DeviceType::parse("server"), Some(DeviceType::Server));
        assert_eq!(DeviceType::parse("router"), Some(DeviceType::Router));
        assert_eq!(DeviceType::parse("database"), Some(DeviceType::Database));
    }
}
