//! CodeLine — a single line of code with line number, used by FileViewer.

use dioxus::prelude::*;

/// A single line of code with line number for the code viewer.
#[component]
pub fn CodeLine(line_number: usize, content: String, is_highlighted: bool) -> Element {
    // Simple syntax coloring based on content prefix
    let line_class =
        if content.trim_start().starts_with("//") || content.trim_start().starts_with('#') {
            "code-comment"
        } else if content.trim_start().starts_with("pub ")
            || content.trim_start().starts_with("fn ")
            || content.trim_start().starts_with("let ")
            || content.trim_start().starts_with("impl ")
            || content.trim_start().starts_with("struct ")
            || content.trim_start().starts_with("enum ")
            || content.trim_start().starts_with("use ")
            || content.trim_start().starts_with("mod ")
        {
            "code-keyword"
        } else if content.trim_start().starts_with('"') {
            "code-string"
        } else {
            ""
        };

    rsx! {
        div {
            class: "code-line",
            "data-highlighted": if is_highlighted { "true" } else { "false" },
            style: { CODE_LINE_STYLES }

            span { class: "line-number", "{line_number}" }
            span {
                class: "line-content {line_class}",
                dangerously_set_inner_html: &escape_html(&content),
            }
        }
    }
}

/// Escape HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

const CODE_LINE_STYLES: &str = "
<style>
  .code-line {
    display: flex;
    padding: 0 0;
    min-height: 21px;
  }

  .code-line:hover {
    background: rgba(255, 255, 255, 0.03);
  }

  .code-line[data-highlighted=\"true\"] {
    background: rgba(108, 92, 231, 0.1);
  }

  .line-number {
    display: inline-block;
    min-width: 48px;
    padding: 0 16px 0 12px;
    text-align: right;
    color: var(--q-text-secondary);
    opacity: 0.5;
    user-select: none;
    border-right: 1px solid var(--q-border);
    margin-right: 12px;
  }

  .line-content {
    white-space: pre;
    tab-size: 4;
  }

  .line-content.code-comment {
    color: #6a9955;
  }

  .line-content.code-keyword {
    color: #569cd6;
  }

  .line-content.code-string {
    color: #ce9178;
  }
</style>
";
