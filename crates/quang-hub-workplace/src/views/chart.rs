//! Chart View — aggregate data from graph nodes into chartable series.
//!
//! Supports bar, line, and pie chart projections.
//! Groups nodes by a dimension (e.g., status, priority, assignee)
//! and counts or sums a metric.

use serde_json::json;

use crate::graph::{HyperGraph, NodeId, NodeKind};
use crate::view::{View, ViewColumn, ViewConfig, ViewItem, ViewProjection};

/// Chart view: aggregate node data into chart series.
pub struct ChartView;

impl ChartView {
    pub fn new() -> Self {
        Self
    }
}

impl View for ChartView {
    fn view_type(&self) -> &str {
        "chart"
    }

    fn display_name(&self) -> &str {
        "Chart"
    }

    fn project(&self, graph: &HyperGraph, config: &ViewConfig) -> ViewProjection {
        let chart_type = config
            .options
            .get("chart_type")
            .and_then(|v| v.as_str())
            .unwrap_or("bar");

        let mut projection = ViewProjection::new("chart", &format!("{} Chart", chart_type));
        projection.description = format!("Chart view ({})", chart_type);

        // Determine what to group by
        let group_by = config
            .options
            .get("group_by")
            .and_then(|v| v.as_str())
            .unwrap_or("status");

        let node_kinds = if config.node_kinds.is_empty() {
            vec![NodeKind::Task]
        } else {
            config.node_kinds.clone()
        };

        // Build aggregation: group_by value -> count
        let mut groups: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();

        for kind in &node_kinds {
            for (node_id, data) in graph.nodes_of_kind(kind) {
                projection.node_ids.push(node_id.clone());
                let key = data
                    .get(group_by)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                *groups.entry(key).or_insert(0) += 1;
            }
        }

        // Build columns (one per group)
        projection.columns = groups
            .iter()
            .map(|(key, count)| ViewColumn {
                id: key.clone(),
                title: key.clone(),
                count: Some(*count),
                color: None,
            })
            .collect();

        // Build items (one per group for the chart)
        projection.items = groups
            .iter()
            .map(|(key, count)| ViewItem {
                node_id: NodeId::new("chart-item"),
                title: key.clone(),
                subtitle: None,
                status: None,
                progress: None,
                assignee: None,
                priority: None,
                start_date: None,
                end_date: None,
                data: json!({
                    "label": key,
                    "value": count,
                    "group_by": group_by,
                }),
            })
            .collect();

        // Store chart-specific data
        projection.view_data = json!({
            "chart_type": chart_type,
            "group_by": group_by,
            "series": [{
                "name": "count",
                "data": groups.iter().map(|(k, v)| json!({"label": k, "value": v})).collect::<Vec<_>>(),
            }],
        });

        projection
    }
}
