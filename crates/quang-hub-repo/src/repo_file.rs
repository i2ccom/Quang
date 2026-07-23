//! RepoFile — file and folder entries in a repository tree.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The type of a filesystem entry in the repo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Submodule,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::File => write!(f, "file"),
            FileType::Directory => write!(f, "dir"),
            FileType::Symlink => write!(f, "symlink"),
            FileType::Submodule => write!(f, "submodule"),
        }
    }
}

/// A file or directory entry in a repository tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoFile {
    /// File/directory name
    pub name: String,
    /// Full path from repo root
    pub path: String,
    /// Entry type
    pub file_type: FileType,
    /// File size in bytes (None for directories)
    pub size: Option<u64>,
    /// SHA of the blob or tree
    pub sha: String,
    /// Last commit that touched this file
    pub last_commit: Option<String>,
    /// Last commit message
    pub last_commit_message: Option<String>,
    /// Last commit author
    pub last_commit_author: Option<String>,
    /// Last commit timestamp
    pub last_commit_at: Option<DateTime<Utc>>,
    /// MIME / media type for syntax highlighting
    pub mime_type: Option<String>,
    /// File extension (e.g. "rs", "ts", "py")
    pub extension: Option<String>,
}

impl RepoFile {
    /// Create a new file entry.
    pub fn new_file(name: &str, path: &str, sha: &str, size: u64) -> Self {
        let ext = name.rsplit('.').nth(1).map(|s| s.to_lowercase());
        Self {
            name: name.to_string(),
            path: path.to_string(),
            file_type: FileType::File,
            size: Some(size),
            sha: sha.to_string(),
            last_commit: None,
            last_commit_message: None,
            last_commit_author: None,
            last_commit_at: None,
            mime_type: detect_mime(name),
            extension: ext,
        }
    }

    /// Create a new directory entry.
    pub fn new_directory(name: &str, path: &str, sha: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            file_type: FileType::Directory,
            size: None,
            sha: sha.to_string(),
            last_commit: None,
            last_commit_message: None,
            last_commit_author: None,
            last_commit_at: None,
            mime_type: None,
            extension: None,
        }
    }

    /// Whether this entry is a directory.
    pub fn is_directory(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Format size as human-readable string.
    pub fn size_display(&self) -> String {
        match self.size {
            None => String::new(),
            Some(bytes) => {
                if bytes < 1024 {
                    format!("{} B", bytes)
                } else if bytes < 1024 * 1024 {
                    format!("{:.1} KB", bytes as f64 / 1024.0)
                } else if bytes < 1024 * 1024 * 1024 {
                    format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
                } else {
                    format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
                }
            }
        }
    }
}

/// A directory listing containing files and subdirectories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoTree {
    /// The path this tree represents
    pub path: String,
    /// The branch or commit SHA
    pub ref_name: String,
    /// Entries in this directory
    pub entries: Vec<RepoFile>,
    /// Total entry count
    pub total_count: usize,
}

/// Detect a MIME type based on file extension for syntax highlighting.
fn detect_mime(name: &str) -> Option<String> {
    let ext = name.rsplit('.').next()?.to_lowercase();
    let mime = match ext.as_str() {
        "rs" => "text/rust",
        "ts" | "tsx" => "text/typescript",
        "js" | "jsx" | "mjs" => "text/javascript",
        "py" => "text/python",
        "go" => "text/go",
        "java" => "text/java",
        "kt" | "kts" => "text/kotlin",
        "swift" => "text/swift",
        "c" | "h" => "text/c",
        "cpp" | "hpp" | "cc" | "cxx" => "text/cpp",
        "rs" => "text/rust",
        "rb" => "text/ruby",
        "php" => "text/php",
        "sql" => "text/sql",
        "sh" | "bash" | "zsh" => "text/shell",
        "yaml" | "yml" => "text/yaml",
        "json" => "application/json",
        "toml" => "text/toml",
        "md" | "markdown" => "text/markdown",
        "html" | "htm" => "text/html",
        "css" | "scss" | "less" => "text/css",
        "svelte" => "text/svelte",
        "vue" => "text/vue",
        "dart" => "text/dart",
        "ex" | "exs" => "text/elixir",
        "clj" | "cljs" | "edn" => "text/clojure",
        "hs" => "text/haskell",
        "ml" | "mli" => "text/ocaml",
        "zig" => "text/zig",
        "nu" => "text/nushell",
        "nix" => "text/nix",
        "txt" | "text" => "text/plain",
        _ => return None,
    };
    Some(mime.to_string())
}
