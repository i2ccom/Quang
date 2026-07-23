//! FileTree — expandable file/folder tree for browsing repository contents.

use dioxus::prelude::*;

/// A tree node representing a file or directory entry.
#[derive(Clone, PartialEq)]
struct TreeNode {
    name: String,
    path: String,
    is_directory: bool,
    children: Vec<TreeNode>,
    expanded: bool,
}

/// Expandable file/folder tree component.
#[component]
pub fn FileTree(
    repo_id: String,
    current_path: String,
    branch: String,
    on_file_select: EventHandler<String>,
) -> Element {
    let tree_data = use_signal(|| {
        vec![
            TreeNode {
                name: "src".to_string(),
                path: "src".to_string(),
                is_directory: true,
                children: vec![
                    TreeNode {
                        name: "main.rs".to_string(),
                        path: "src/main.rs".to_string(),
                        is_directory: false,
                        children: vec![],
                        expanded: false,
                    },
                    TreeNode {
                        name: "lib.rs".to_string(),
                        path: "src/lib.rs".to_string(),
                        is_directory: false,
                        children: vec![],
                        expanded: false,
                    },
                ],
                expanded: true,
            },
            TreeNode {
                name: "Cargo.toml".to_string(),
                path: "Cargo.toml".to_string(),
                is_directory: false,
                children: vec![],
                expanded: false,
            },
            TreeNode {
                name: "README.md".to_string(),
                path: "README.md".to_string(),
                is_directory: false,
                children: vec![],
                expanded: false,
            },
        ]
    });

    rsx! {
        div { class: "file-tree",
            style: { FILE_TREE_STYLES }
            div { class: "file-tree-header",
                span { "Files" }
            }
            div { class: "file-tree-content",
                for node in tree_data.read().iter() {
                    TreeNodeComponent {
                        node: node.clone(),
                        depth: 0,
                        on_file_select: on_file_select.clone(),
                    }
                }
            }
        }
    }
}

/// Recursive tree node component.
#[component]
fn TreeNodeComponent(
    node: TreeNode,
    depth: usize,
    on_file_select: EventHandler<String>,
) -> Element {
    let expanded = use_signal(|| node.expanded);

    rsx! {
        div { class: "tree-node-wrapper",
            div {
                class: "tree-node",
                style: "padding-left: {depth * 16 + 8}px;",
                onclick: move |_| {
                    if node.is_directory {
                        expanded.set(!expanded.read());
                    } else {
                        on_file_select.call(node.path.clone());
                    }
                },

                if node.is_directory {
                    span { class: "tree-toggle",
                        if *expanded.read() { "▼" } else { "▶" }
                    }
                    span { class: "tree-icon", "📁" }
                } else {
                    span { class: "tree-toggle", style: "visibility: hidden;", "▶" }
                    span { class: "tree-icon", get_file_icon(&node.name) }
                }

                span { class: "tree-name", "{node.name}" }
            }

            if node.is_directory && *expanded.read() {
                div { class: "tree-children",
                    for child in node.children.iter() {
                        TreeNodeComponent {
                            node: child.clone(),
                            depth: depth + 1,
                            on_file_select: on_file_select.clone(),
                        }
                    }
                }
            }
        }
    }
}

/// Get an appropriate emoji icon for a file.
fn get_file_icon(name: &str) -> &'static str {
    if name.ends_with(".rs") {
        "🦀"
    } else if name.ends_with(".ts") || name.ends_with(".tsx") {
        "🟦"
    } else if name.ends_with(".js") || name.ends_with(".jsx") {
        "🟨"
    } else if name.ends_with(".py") {
        "🐍"
    } else if name.ends_with(".toml") {
        "⚙"
    } else if name.ends_with(".md") {
        "📝"
    } else if name.ends_with(".json") {
        "📋"
    } else if name.ends_with(".css") || name.ends_with(".scss") {
        "🎨"
    } else if name.ends_with(".html") {
        "🌐"
    } else if name == "Dockerfile" || name.ends_with("Dockerfile") {
        "🐳"
    } else if name.starts_with(".github") {
        "🤖"
    } else {
        "📄"
    }
}

const FILE_TREE_STYLES: &str = "
<style>
  .file-tree {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .file-tree-header {
    padding: 10px 12px;
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--q-text-secondary);
    border-bottom: 1px solid var(--q-border);
  }

  .file-tree-content {
    flex: 1;
    overflow-y: auto;
    padding: 4px 0;
  }

  .tree-node {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 12px;
    font-size: 13px;
    cursor: pointer;
    border-radius: 4px;
    margin: 0 4px;
    white-space: nowrap;
  }

  .tree-node:hover {
    background: var(--q-surface-hover);
  }

  .tree-toggle {
    font-size: 8px;
    width: 12px;
    flex-shrink: 0;
    color: var(--q-text-secondary);
  }

  .tree-icon {
    font-size: 14px;
    flex-shrink: 0;
  }

  .tree-name {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tree-children {
    display: flex;
    flex-direction: column;
  }
</style>
";
