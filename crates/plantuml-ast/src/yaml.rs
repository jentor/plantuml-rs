//! AST типы для YAML диаграмм
//!
//! YAML диаграммы визуализируют YAML данные в виде дерева.
//! Они переиспользуют структуру JSON, так как YAML — надмножество JSON.
//!
//! Синтаксис PlantUML:
//! ```text
//! @startyaml
//! name: John
//! age: 30
//! address:
//!   city: New York
//!   zip: 10001
//! hobbies:
//!   - reading
//!   - gaming
//! @endyaml
//! ```

use serde::{Deserialize, Serialize};

use crate::common::DiagramMetadata;
use crate::json::JsonNode;

/// YAML диаграмма (использует ту же структуру что и JSON)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct YamlDiagram {
    /// Корневой узел (использует JsonNode для совместимости)
    pub root: Option<JsonNode>,
    /// Метаданные диаграммы
    pub metadata: DiagramMetadata,
    /// Подсветка определённых путей (highlight)
    pub highlights: Vec<String>,
}

impl YamlDiagram {
    /// Создаёт новую пустую YAML диаграмму
    pub fn new() -> Self {
        Self {
            root: None,
            metadata: DiagramMetadata::default(),
            highlights: Vec::new(),
        }
    }

    /// Создаёт YAML диаграмму с корневым узлом
    pub fn with_root(root: JsonNode) -> Self {
        Self {
            root: Some(root),
            metadata: DiagramMetadata::default(),
            highlights: Vec::new(),
        }
    }

    /// Возвращает общее количество узлов
    pub fn node_count(&self) -> usize {
        self.root.as_ref().map_or(0, |r| r.count_all())
    }

    /// Возвращает максимальную глубину
    pub fn max_depth(&self) -> usize {
        self.root.as_ref().map_or(0, |r| r.max_depth())
    }
}

impl Default for YamlDiagram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_yaml_diagram() {
        let root = JsonNode::object(
            None,
            vec![
                JsonNode::string(Some("name".into()), "John"),
                JsonNode::number(Some("age".into()), 30.0),
            ],
        );

        let diagram = YamlDiagram::with_root(root);
        assert_eq!(diagram.node_count(), 3);
        assert_eq!(diagram.max_depth(), 2);
    }

    #[test]
    fn test_nested_yaml() {
        let address = JsonNode::object(
            Some("address".into()),
            vec![JsonNode::string(Some("city".into()), "New York")],
        );

        let root = JsonNode::object(None, vec![address]);
        let diagram = YamlDiagram::with_root(root);

        assert_eq!(diagram.max_depth(), 3);
    }
}
