//! FileViewer — code viewer with line numbers and syntax highlighting.

use dioxus::prelude::*;

use crate::web::components::code_line::CodeLine;

/// Code file viewer with line numbers and syntax highlighting.
#[component]
pub fn FileViewer(
    repo_id: String,
    file_path: String,
    branch: String,
) -> Element {
    let lines = use_signal(|| {
        // In production, fetch file content from API
        // For now, provide mock content based on file extension
        let extension = file_path.rsplit('.').next().unwrap_or("txt");
        generate_mock_content(&file_path, extension)
    });
    let start_line = use_signal(|| 1usize);

    rsx! {
        div { class: "file-viewer",
            style: { FILE_VIEWER_STYLES }

            // ── Toolbar ──
            div { class: "file-viewer-toolbar",
                span { class: "file-viewer-path", "{file_path}" }
                div { class: "file-viewer-actions",
                    span { class: "file-viewer-line-count",
                        "{lines.read().len()} lines"
                    }
                    button { class: "toolbar-btn", "📋 Copy" }
                    button { class: "toolbar-btn", "🔗 Permalink" }
                    button { class: "toolbar-btn", "📥 Raw" }
                }
            }

            // ── Code content ──
            div { class: "file-viewer-content",
                div { class: "code-lines",
                    for (i, line) in lines.read().iter().enumerate() {
                        CodeLine {
                            key: "{i}",
                            line_number: start_line() + i,
                            content: line.clone(),
                            is_highlighted: false,
                        }
                    }
                }
            }
        }
    }
}

/// Generate mock file content for display purposes.
fn generate_mock_content(file_path: &str, extension: &str) -> Vec<String> {
    match extension {
        "rs" => vec![
            "use serde::{Deserialize, Serialize};".to_string(),
            "use chrono::{DateTime, Utc};".to_string(),
            "use uuid::Uuid;".to_string(),
            "".to_string(),
            "/// A sample struct for demonstration.".to_string(),
            "#[derive(Debug, Clone, Serialize, Deserialize)]".to_string(),
            "pub struct Sample {".to_string(),
            "    pub id: String,".to_string(),
            "    pub name: String,".to_string(),
            "    pub created_at: DateTime<Utc>,".to_string(),
            "}".to_string(),
            "".to_string(),
            "impl Sample {".to_string(),
            "    pub fn new(name: &str) -> Self {".to_string(),
            "        Self {".to_string(),
            "            id: Uuid::new_v4().to_string(),".to_string(),
            "            name: name.to_string(),".to_string(),
            "            created_at: Utc::now(),".to_string(),
            "        }".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ],
        "toml" => vec![
            "[package]".to_string(),
            "name = \"quang-hub-repo\"".to_string(),
            "version = \"0.1.0\"".to_string(),
            "edition = \"2021\"".to_string(),
            "".to_string(),
            "[dependencies]".to_string(),
            "serde = { version = \"1\", features = [\"derive\"] }".to_string(),
            "chrono = { version = \"0.4\", features = [\"serde\"] }".to_string(),
            "uuid = { version = \"1\", features = [\"v4\", \"serde\"] }".to_string(),
        ],
        "md" => vec![
            "# Project Title".to_string(),
            "".to_string(),
            "## Description".to_string(),
            "".to_string(),
            "This is a sample project. Replace this with your project description.".to_string(),
            "".to_string(),
            "## Getting Started".to_string(),
            "".to_string(),
            "### Prerequisites".to_string(),
            "".to_string(),
            "- Rust 1.75+".to_string(),
            "- Node.js 20+".to_string(),
            "".to_string(),
            "### Installation".to_string(),
            "".to_string(),
            "```bash".to_string(),
            "git clone https://github.com/example/project.git".to_string(),
            "cd project".to_string(),
            "cargo build".to_string(),
            "```".to_string(),
        ],
        _ => vec![
            "// File: ".to_string() + file_path,
            "// This file could not be previewed.".to_string(),
            "// Raw download is available.".to_string(),
        ],
    }
}

const FILE_VIEWER_STYLES: &str = "
<style>
  .file-viewer {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .file-viewer-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    border-bottom: 1px solid var(--q-border);
    background: var(--q-bg);
  }

  .file-viewer-path {
    font-size: 13px;
    font-family: monospace;
    color: var(--q-text);
  }

  .file-viewer-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .file-viewer-line-count {
    font-size: 12px;
    color: var(--q-text-secondary);
  }

  .toolbar-btn {
    background: transparent;
    border: 1px solid var(--q-border);
    padding: 4px 10px;
    font-size: 12px;
    border-radius: 4px;
    color: var(--q-text-secondary);
    cursor: pointer;
  }

  .toolbar-btn:hover {
    color: var(--q-text);
    background: var(--q-surface-hover);
  }

  .file-viewer-content {
    flex: 1;
    overflow: auto;
    background: var(--q-bg);
  }

  .code-lines {
    font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace;
    font-size: 13px;
    line-height: 1.6;
    min-width: 100%;
  }
</style>
";
