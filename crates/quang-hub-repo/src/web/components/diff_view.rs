//! DiffView — side-by-side diff viewer for commits and PRs.

use dioxus::prelude::*;

/// A single diff hunk showing line changes.
#[derive(Clone, PartialEq)]
struct DiffLine {
    line_type: DiffLineType,
    old_line_number: Option<u32>,
    new_line_number: Option<u32>,
    content: String,
}

#[derive(Clone, PartialEq)]
enum DiffLineType {
    Added,
    Removed,
    Context,
}

/// Side-by-side diff view component.
#[component]
pub fn DiffView(
    repo_id: String,
    old_sha: Option<String>,
    new_sha: String,
    file_path: Option<String>,
) -> Element {
    let diff_lines = use_signal(|| {
        // Mock diff data
        generate_mock_diff()
    });

    rsx! {
        div { class: "diff-view",
            style: { DIFF_VIEW_STYLES }

            // ── File header ──
            if let Some(ref path) = file_path {
                div { class: "diff-file-header",
                    span { class: "diff-file-path", "{path}" }
                    span { class: "diff-stats",
                        span { class: "diff-additions",
                            "+{diff_lines.read().iter().filter(|l| l.line_type == DiffLineType::Added).count()}"
                        }
                        span { class: "diff-deletions",
                            "-{diff_lines.read().iter().filter(|l| l.line_type == DiffLineType::Removed).count()}"
                        }
                    }
                }
            }

            // ── Unified diff ──
            div { class: "diff-content",
                for (i, line) in diff_lines.read().iter().enumerate() {
                    div {
                        class: "diff-line",
                        class: if line.line_type == DiffLineType::Added { "diff-added" }
                            else if line.line_type == DiffLineType::Removed { "diff-removed" }
                            else { "diff-context" },

                        span { class: "diff-old-line",
                            if let Some(n) = line.old_line_number { "{n}" }
                        }
                        span { class: "diff-new-line",
                            if let Some(n) = line.new_line_number { "{n}" }
                        }
                        span { class: "diff-prefix",
                            if line.line_type == DiffLineType::Added { "+" }
                            else if line.line_type == DiffLineType::Removed { "-" }
                            else { " " }
                        }
                        span { class: "diff-text",
                            "{line.content}"
                        }
                    }
                }
            }
        }
    }
}

/// Generate mock diff data for display.
fn generate_mock_diff() -> Vec<DiffLine> {
    vec![
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(1),
            new_line_number: Some(1),
            content: "use serde::{Deserialize, Serialize};".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(2),
            new_line_number: Some(2),
            content: "use chrono::{DateTime, Utc};".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(3),
            new_line_number: Some(3),
            content: String::new(),
        },
        DiffLine {
            line_type: DiffLineType::Removed,
            old_line_number: Some(4),
            new_line_number: None,
            content: "// Old deprecated function".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Removed,
            old_line_number: Some(5),
            new_line_number: None,
            content: "pub fn old_function() -> String {".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Removed,
            old_line_number: Some(6),
            new_line_number: None,
            content: "    \"old\".to_string()".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Removed,
            old_line_number: Some(7),
            new_line_number: None,
            content: "}".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(4),
            content: "/// New improved function".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(5),
            content: "pub fn new_function() -> String {".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(6),
            content: "    \"new and improved\".to_string()".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(7),
            content: "}".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(8),
            new_line_number: Some(8),
            content: String::new(),
        },
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(9),
            new_line_number: Some(9),
            content: "#[cfg(test)]".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(10),
            new_line_number: Some(10),
            content: "mod tests {".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(11),
            content: "    use super::*;".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(12),
            content: "".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(13),
            content: "    #[test]".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(14),
            content: "    fn test_new_function() {".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(15),
            content: "        assert_eq!(new_function(), \"new and improved\");".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Added,
            old_line_number: None,
            new_line_number: Some(16),
            content: "    }".to_string(),
        },
        DiffLine {
            line_type: DiffLineType::Context,
            old_line_number: Some(11),
            new_line_number: Some(17),
            content: "}".to_string(),
        },
    ]
}

const DIFF_VIEW_STYLES: &str = "
<style>
  .diff-view {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    overflow: hidden;
  }

  .diff-file-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    background: var(--q-surface);
    border-bottom: 1px solid var(--q-border);
  }

  .diff-file-path {
    font-size: 13px;
    font-family: monospace;
  }

  .diff-stats {
    display: flex;
    gap: 8px;
    font-size: 12px;
    font-family: monospace;
  }

  .diff-additions {
    color: var(--q-success);
  }

  .diff-deletions {
    color: var(--q-danger);
  }

  .diff-content {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 12px;
    line-height: 1.5;
    overflow-x: auto;
  }

  .diff-line {
    display: flex;
    min-height: 20px;
  }

  .diff-added {
    background: rgba(81, 207, 102, 0.1);
  }

  .diff-removed {
    background: rgba(255, 107, 107, 0.1);
  }

  .diff-context {
    background: transparent;
  }

  .diff-old-line,
  .diff-new-line {
    min-width: 36px;
    text-align: right;
    padding: 0 8px;
    color: var(--q-text-secondary);
    opacity: 0.4;
    user-select: none;
  }

  .diff-prefix {
    min-width: 18px;
    text-align: center;
    font-weight: 600;
  }

  .diff-added .diff-prefix { color: var(--q-success); }
  .diff-removed .diff-prefix { color: var(--q-danger); }

  .diff-text {
    flex: 1;
    white-space: pre;
    padding-right: 16px;
  }
</style>
";
