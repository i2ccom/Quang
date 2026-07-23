//! Kanban View — columns grouped by status with cards.
//!
//! Each node becomes a card. Cards are grouped into columns by their status field.
//! This is the classic project management board view.

use serde_json::Value;

use crate::graph::{HyperGraph, NodeKind};
use crate::view::{View, ViewColumn, ViewConfig, ViewItem, ViewProjection};

/// Kanban view: nodes as cards grouped by status.
pub struct KanbanView;

impl KanbanView {
    pub fn new() -> Self {
        Self
    }
}

impl View for KanbanView {
    fn view_type(&self) -> &str {
        "kanban"
    }

    fn display_name(&self) -> &str {
        "Kanban Board"
    }

    fn project(&self, graph: &HyperGraph, config: &ViewConfig) -> ViewProjection {
        let mut projection = ViewProjection::new("kanban", "Kanban Board");
        projection.description = "Task board grouped by status".to_string();

        // Define standard Kanban columns
        let column_defs = vec![
            ("backlog", "Backlog", "#9ca3af"),
            ("ready", "Ready", "#60a5fa"),
            ("in_progress", "In Progress", "#f59e0b"),
            ("in_review", "In Review", "#8b5cf6"),
            ("done", "Done", "#22c55e"),
        ];

        let mut columns: Vec<ViewColumn> = column_defs
            .iter()
            .map(|(id, title, color)| ViewColumn {
                id: id.to_string(),
                title: title.to_string(),
                count: Some(0),
                color: Some(color.to_string()),
            })
            .collect();

        // Collect items grouped by status
        let node_kinds = if config.node_kinds.is_empty() {
            vec![NodeKind::Task]
        } else {
            config.node_kinds.clone()
        };

        // Build a map: status -> items
        let mut status_map: std::collections::HashMap<String, Vec<ViewItem>> = column_defs
            .iter()
            .map(|(id, _, _)| (id.to_string(), Vec::new()))
            .collect();

        for kind in &node_kinds {
            for (node_id, data) in graph.nodes_of_kind(kind) {
                let status =
                    get_string_field(data, "status").unwrap_or_else(|| "backlog".to_string());

                projection.node_ids.push(node_id.clone());

                let item = ViewItem {
                    node_id: node_id.clone(),
                    title: get_string_field(data, "title")
                        .or_else(|| get_string_field(data, "name"))
                        .unwrap_or_else(|| node_id.to_string()),
                    subtitle: get_string_field(data, "description"),
                    status: Some(status.clone()),
                    progress: data.get("progress").and_then(|v| v.as_f64()),
                    assignee: get_assignee(data),
                    priority: get_string_field(data, "priority"),
                    start_date: None,
                    end_date: None,
                    data: data.clone(),
                };

                // Place in the right column (or "backlog" as fallback)
                let entry = status_map.entry(status).or_insert_with(Vec::new);
                entry.push(item);
            }
        }

        // Flatten: items in column order, update counts
        for col in &mut columns {
            if let Some(items) = status_map.get(&col.id) {
                col.count = Some(items.len());
            }
        }

        for (_, items) in &status_map {
            projection.items.extend(items.iter().cloned());
        }

        // Apply limit
        if let Some(limit) = config.limit {
            projection.items.truncate(limit);
        }

        projection.columns = columns;
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
