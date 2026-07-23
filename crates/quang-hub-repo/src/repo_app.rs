//! RepoAppManifest — the `qh.app` adaptive repo application manifest.
//!
//! A `qh.app` manifest describes how a repository should be rendered as an
//! interactive application within QuangHub. This enables "adaptive repo apps"
//! where a repo can define its own UI — dashboards, playgrounds, visualizers,
//! calculators, or full CRUD interfaces — that render inside the QuangHub
//! repo browser without needing a separate frontend deployment.

use serde::{Deserialize, Serialize};

/// A `qh.app` manifest file (typically `qh.app.json` or `.qh.app.toml`)
/// that defines an adaptive application for the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoAppManifest {
    /// Manifest schema version
    pub version: String,
    /// App identifier (kebab-case, unique per org)
    pub id: String,
    /// Display name
    pub name: String,
    /// Short description
    pub description: Option<String>,
    /// App icon (emoji or URL)
    pub icon: Option<String>,
    /// Entry point definitions
    pub entries: Vec<AppEntry>,
    /// Navigation items (tabs shown in the app viewer)
    pub navigation: Vec<AppNavItem>,
    /// The render mode for this app
    #[serde(rename = "renderMode")]
    pub render_mode: AppRenderMode,
    /// Permissions the app requires
    pub permissions: Option<Vec<String>>,
    /// Theme overrides
    pub theme: Option<AppTheme>,
    /// Data sources this app consumes
    pub sources: Option<Vec<AppDataSource>>,
    /// Actions this app exposes
    pub actions: Option<Vec<AppAction>>,
}

/// An app entry point (a file path relative to repo root).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppEntry {
    /// Entry point ID
    pub id: String,
    /// File path relative to repo root
    pub path: String,
    /// Entry type: component, page, or script
    pub entry_type: AppEntryType,
    /// Label in the UI
    pub label: Option<String>,
}

/// Type of app entry point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppEntryType {
    /// A Dioxus component to render inline
    Component,
    /// A full page route
    Page,
    /// A sidecar script to execute
    Script,
    /// Markdown content to render
    Markdown,
    /// An HTML file to embed
    Html,
}

impl std::fmt::Display for AppEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEntryType::Component => write!(f, "component"),
            AppEntryType::Page => write!(f, "page"),
            AppEntryType::Script => write!(f, "script"),
            AppEntryType::Markdown => write!(f, "markdown"),
            AppEntryType::Html => write!(f, "html"),
        }
    }
}

/// A navigation tab within the app viewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNavItem {
    /// Tab label
    pub label: String,
    /// Entry point ID this tab points to
    pub entry: String,
    /// Icon (emoji or icon name)
    pub icon: Option<String>,
}

/// How the app is rendered in the repo viewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppRenderMode {
    /// Rendered inline within the file viewer panel
    Inline,
    /// Takes over the full repo detail page
    FullPage,
    /// Rendered as a sidebar panel
    Sidebar,
    /// Rendered as a modal dialog
    Modal,
    /// Rendered as a separate tab in the repo detail view
    Tab,
}

impl std::fmt::Display for AppRenderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppRenderMode::Inline => write!(f, "inline"),
            AppRenderMode::FullPage => write!(f, "full_page"),
            AppRenderMode::Sidebar => write!(f, "sidebar"),
            AppRenderMode::Modal => write!(f, "modal"),
            AppRenderMode::Tab => write!(f, "tab"),
        }
    }
}

/// Theme overrides for the app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppTheme {
    pub primary: Option<String>,
    pub background: Option<String>,
    pub surface: Option<String>,
    pub text: Option<String>,
}

/// A data source the app consumes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDataSource {
    pub kind: String,
    pub path: Option<String>,
    pub url: Option<String>,
}

/// An action the app exposes to the QuangHub UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAction {
    pub id: String,
    pub label: String,
    pub action_type: String,
    pub path: String,
}

impl RepoAppManifest {
    /// Find an entry by its ID.
    pub fn find_entry(&self, id: &str) -> Option<&AppEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Get the default entry (first one, or the one labelled "main").
    pub fn default_entry(&self) -> Option<&AppEntry> {
        self.entries
            .iter()
            .find(|e| e.id == "main" || e.id == "index")
            .or_else(|| self.entries.first())
    }
}

/// A parsed collection of all qh.app manifests found in a repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoAppsCollection {
    /// All discovered manifests
    pub apps: Vec<RepoAppManifest>,
    /// The repo ID these apps belong to
    pub repo_id: String,
}
