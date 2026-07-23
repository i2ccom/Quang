//! Built-in view implementations for the Workplace collaboration graph.
//!
//! Each view is a graph projection that renders the same underlying data
//! in a different layout. Views are registered in the ViewRegistry.

pub mod chart;
pub mod gantt;
pub mod kanban;
pub mod table;

pub use chart::ChartView;
pub use gantt::GanttView;
pub use kanban::KanbanView;
pub use table::TableView;

/// Register all built-in views into a ViewRegistry.
pub fn register_builtin_views(registry: &mut crate::view::ViewRegistry) {
    registry.register(Box::new(TableView::new()));
    registry.register(Box::new(KanbanView::new()));
    registry.register(Box::new(ChartView::new()));
    registry.register(Box::new(GanttView::new()));
}
