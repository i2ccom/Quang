//! Reusable UI components for the Workplace web module.
//!
//! Each component is a pure Dioxus functional component that can be
//! composed into pages and other components.

pub mod agent_chat_panel;
pub mod agent_design_panel;
pub mod agent_goal_panel;
pub mod agent_task_panel;
pub mod channel_list;
pub mod chat_panel;
pub mod goal_progress;
pub mod kanban_column;
pub mod project_card;
pub mod task_card;
pub mod team_card;
pub mod workspace_card;

pub use agent_chat_panel::AgentChatPanel;
pub use agent_design_panel::AgentDesignPanel;
pub use agent_goal_panel::AgentGoalPanel;
pub use agent_task_panel::AgentTaskPanel;
pub use channel_list::ChannelList;
pub use chat_panel::ChatPanel;
pub use goal_progress::GoalProgress;
pub use kanban_column::KanbanColumn;
pub use project_card::ProjectCard;
pub use task_card::TaskCard;
pub use team_card::TeamCard;
pub use workspace_card::WorkspaceCard;
