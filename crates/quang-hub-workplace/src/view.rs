//! View — graph projections for the collaboration graph.
//!
//! Views select subsets of nodes and edges from the HyperGraph and
//! project them into a specific layout: Table, Kanban, Chart, or Gantt.
//! New view types can be registered at runtime for extensibility.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::graph::{HyperGraph, NodeId, NodeKind};

/// Unique identifier for a view type.
pub type ViewTypeId = String;

/// Configuration for any view. Each view type interprets this differently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewConfig {
    /// What node kinds to include
    pub node_kinds: Vec<NodeKind>,
    /// What edge kinds to traverse
    pub edge_kinds: Vec<String>,
    /// Sort field (e.g., "status", "priority", "created_at")
    pub sort_by: Option<String>,
    /// Sort direction
    pub sort_desc: bool,
    /// Filter expression (future: could be a simple DSL)
    pub filter: Option<String>,
    /// Maximum items to show
    pub limit: Option<usize>,
    /// View-specific options (extensible)
    pub options: HashMap<String, serde_json::Value>,
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self {
            node_kinds: Vec::new(),
            edge_kinds: Vec::new(),
            sort_by: None,
            sort_desc: false,
            filter: None,
            limit: None,
            options: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// View trait
// ---------------------------------------------------------------------------

/// A View is a projection of the HyperGraph into a specific layout.
/// Views can be registered at runtime, allowing agents to add new visualizations.
pub trait View: Send + Sync {
    /// The unique type identifier (e.g., "table", "kanban", "chart", "gantt").
    fn view_type(&self) -> &str;

    /// Human-readable display name.
    fn display_name(&self) -> &str;

    /// Compute a projection from the graph given the config.
    fn project(&self, graph: &HyperGraph, config: &ViewConfig) -> ViewProjection;
}

// ---------------------------------------------------------------------------
// View projection result
// ---------------------------------------------------------------------------

/// The result of projecting the graph through a view.
/// This is a generic structure that each view type populates differently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewProjection {
    pub view_type: String,
    pub title: String,
    pub description: String,
    /// The node IDs included in this projection
    pub node_ids: Vec<NodeId>,
    /// Abstract "columns" or "groups" — interpreted per view type
    pub columns: Vec<ViewColumn>,
    /// Abstract "rows" or "items" — interpreted per view type
    pub items: Vec<ViewItem>,
    /// View-type-specific data blob
    pub view_data: serde_json::Value,
}

impl ViewProjection {
    pub fn new(view_type: &str, title: &str) -> Self {
        Self {
            view_type: view_type.to_string(),
            title: title.to_string(),
            description: String::new(),
            node_ids: Vec::new(),
            columns: Vec::new(),
            items: Vec::new(),
            view_data: serde_json::Value::Null,
        }
    }
}

/// A column/group header in a view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewColumn {
    pub id: String,
    pub title: String,
    /// Optional count of items in this column
    pub count: Option<usize>,
    /// Optional color/status indicator
    pub color: Option<String>,
}

/// A single item/row in a view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewItem {
    pub node_id: NodeId,
    pub title: String,
    pub subtitle: Option<String>,
    pub status: Option<String>,
    pub progress: Option<f64>,
    pub assignee: Option<String>,
    pub priority: Option<String>,
    /// Timestamps for Gantt chart
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    /// Raw data for custom rendering
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// ViewRegistry
// ---------------------------------------------------------------------------

/// Registry of available view types. Allows agents to register new views.
#[derive(Default)]
pub struct ViewRegistry {
    views: HashMap<ViewTypeId, Box<dyn View>>,
}

impl ViewRegistry {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
        }
    }

    /// Register a new view type.
    pub fn register(&mut self, view: Box<dyn View>) {
        let id = view.view_type().to_string();
        self.views.insert(id, view);
    }

    /// Get a view by type.
    pub fn get(&self, view_type: &str) -> Option<&dyn View> {
        self.views.get(view_type).map(|v| v.as_ref())
    }

    /// List all registered view types.
    pub fn available_views(&self) -> Vec<ViewTypeId> {
        self.views.keys().cloned().collect()
    }

    /// Project the graph through a specific view.
    pub fn project(
        &self,
        view_type: &str,
        graph: &HyperGraph,
        config: &ViewConfig,
    ) -> Option<ViewProjection> {
        self.get(view_type).map(|v| v.project(graph, config))
    }
}
