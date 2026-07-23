//! Table View — a sortable, filterable tabular projection of graph nodes.
//!
//! Each node becomes a row. Selected fields become columns.
//! Columns are derived from the node's JSON Value fields.

use serde_json::Value;

use crate::graph::HyperGraph;
use crate::view::{View, ViewColumn, ViewConfig, ViewItem, ViewProjection};

/// Table view: nodes as rows, fields as columns.
pub struct TableView;

impl TableView {
    pub fn new() -> Self {
        Self
    }
}

impl View for TableView {
    fn view_type(&self) -> &str {
        "table"
    }

    fn display_name(&self) -> &str {
        "Table"
    }

    fn project(&self, graph: &HyperGraph, config: &ViewConfig) -> ViewProjection {
        let mut projection = ViewProjection::new("table", "Table View");
        projection.description = "Tabular view of nodes and their fields".to_string();

        // Columns derived from common fields
        let columns = vec![
            ViewColumn {
                id: "title".into(),
                title: "Title".into(),
                count: None,
                color: None,
            },
            ViewColumn {
                id: "status".into(),
                title: "Status".into(),
                count: None,
                color: None,
            },
            ViewColumn {
                id: "priority".into(),
                title: "Priority".into(),
                count: None,
                color: None,
            },
            ViewColumn {
                id: "assignee".into(),
                title: "Assignee".into(),
                count: None,
                color: None,
            },
            ViewColumn {
                id: "created_at".into(),
                title: "Created".into(),
                count: None,
                color: None,
            },
            ViewColumn {
                id: "updated_at".into(),
                title: "Updated".into(),
                count: None,
                color: None,
            },
        ];
        projection.columns = columns;

        // Filter nodes by kind
        let node_kinds = if config.node_kinds.is_empty() {
            vec![
                crate::graph::NodeKind::Task,
                crate::graph::NodeKind::Project,
                crate::graph::NodeKind::Goal,
            ]
        } else {
            config.node_kinds.clone()
        };

        for kind in &node_kinds {
            for (node_id, data) in graph.nodes_of_kind(kind) {
                projection.node_ids.push(node_id.clone());

                let item = ViewItem {
                    node_id: node_id.clone(),
                    title: get_string_field(data, "title")
                        .or_else(|| get_string_field(data, "name"))
                        .unwrap_or_else(|| node_id.to_string()),
                    subtitle: get_string_field(data, "description"),
                    status: get_string_field(data, "status"),
                    progress: data.get("progress").and_then(|v| v.as_f64()),
                    assignee: get_assignee(data),
                    priority: get_string_field(data, "priority"),
                    start_date: None,
                    end_date: None,
                    data: data.clone(),
                };
                projection.items.push(item);

                // Apply limit
                if let Some(limit) = config.limit {
                    if projection.items.len() >= limit {
                        break;
                    }
                }
            }
        }

        projection
    }
}

fn get_string_field(data: &Value, field: &str) -> Option<String> {
    data.get(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn get_assignee(data: &Value) -> Option<String> {
    data.get("assignee").and_then(|v| {
        if let Some(s) = v.as_str() {
            return Some(s.to_string());
        }
        v.get("Human")
            .or_else(|| v.get("Agent"))
            .and_then(|inner| inner.as_str())
            .map(|s| s.to_string())
    })
}
