//! Gantt Chart View — timeline bars for tasks and milestones.
//!
//! Projects tasks and milestones along a timeline.
//! Each item with a start_date and end_date becomes a bar.
//! Dependencies (DependsOn edges) are rendered as arrows.

use serde_json::{json, Value};

use crate::graph::{EdgeKind, HyperGraph, NodeKind};
use crate::view::{View, ViewColumn, ViewConfig, ViewItem, ViewProjection};

/// Gantt chart view: timeline bars for scheduled items.
pub struct GanttView;

impl GanttView {
    pub fn new() -> Self {
        Self
    }
}

impl View for GanttView {
    fn view_type(&self) -> &str {
        "gantt"
    }

    fn display_name(&self) -> &str {
        "Gantt Chart"
    }

    fn project(&self, graph: &HyperGraph, config: &ViewConfig) -> ViewProjection {
        let mut projection = ViewProjection::new("gantt", "Gantt Chart");
        projection.description = "Timeline view of scheduled tasks".to_string();

        let node_kinds = if config.node_kinds.is_empty() {
            vec![NodeKind::Task, NodeKind::Project]
        } else {
            config.node_kinds.clone()
        };

        // Single column for Gantt
        projection.columns = vec![ViewColumn {
            id: "timeline".into(),
            title: "Timeline".into(),
            count: None,
            color: None,
        }];

        for kind in &node_kinds {
            for (node_id, data) in graph.nodes_of_kind(kind) {
                projection.node_ids.push(node_id.clone());

                let start = get_string_field(data, "start_date")
                    .or_else(|| get_string_field(data, "created_at"));
                let end = get_string_field(data, "end_date")
                    .or_else(|| get_string_field(data, "due_date"))
                    .or_else(|| get_string_field(data, "completed_at"));

                // Only include items with at least a start date
                if start.is_none() {
                    continue;
                }

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
                    start_date: start,
                    end_date: end,
                    data: data.clone(),
                };
                projection.items.push(item);
            }
        }

        // Collect dependency edges for the view_data
        let dependencies: Vec<Value> = graph
            .edges
            .iter()
            .filter(|e| e.kind == EdgeKind::DependsOn)
            .map(|e| {
                json!({
                    "from": e.source.to_string(),
                    "to": e.target.to_string(),
                    "label": e.metadata.get("type").cloned().unwrap_or_default(),
                })
            })
            .collect();

        projection.view_data = json!({
            "dependencies": dependencies,
            "item_count": projection.items.len(),
        });

        projection
    }
}

fn get_string_field(data: &Value, field: &str) -> Option<String> {
    data.get(field).and_then(|v| v.as_str()).map(|s| s.to_string())
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
