//! Page-level components for the Workplace web UI.
//!
//! Each page is a full Dioxus component that assembles layout + components
//! into a complete view. Pages are registered with the Dioxus Router.

pub mod channel_view;
pub mod goal_board;
pub mod login;
pub mod review_board;
pub mod task_board;
pub mod welcome;
pub mod workspace_dashboard;

pub use channel_view::ChannelView;
pub use goal_board::GoalBoard;
pub use login::Login;
pub use review_board::ReviewBoard;
pub use task_board::TaskBoard;
pub use welcome::Welcome;
pub use workspace_dashboard::WorkspaceDashboard;
