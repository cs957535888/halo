use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

#[derive(Debug, thiserror::Error, Serialize)]
pub enum ListDirError {
    #[error("path is not a directory: {0}")]
    NotADirectory(String),
    #[error("io: {0}")]
    Io(String),
}

pub fn list_dir(path: &str) -> Result<Vec<FileNode>, ListDirError> {
    let p = Path::new(path);
    if !p.is_dir() {
        return Err(ListDirError::NotADirectory(path.to_string()));
    }
    let mut out = Vec::new();
    let entries = fs::read_dir(p).map_err(|e| ListDirError::Io(e.to_string()))?;
    for entry in entries.flatten() {
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') { continue; } // skip hidden in M1
        out.push(FileNode {
            name,
            path: entry.path().to_string_lossy().to_string(),
            is_dir: meta.is_dir(),
        });
    }
    out.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn lists_dir_with_dirs_first_then_alpha() {
        let d = tempdir().unwrap();
        File::create(d.path().join("zfile.txt")).unwrap();
        File::create(d.path().join("afile.txt")).unwrap();
        std::fs::create_dir(d.path().join("sub")).unwrap();
        let out = list_dir(d.path().to_str().unwrap()).unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].name, "sub");
        assert_eq!(out[0].is_dir, true);
        assert_eq!(out[1].name, "afile.txt");
        assert_eq!(out[2].name, "zfile.txt");
    }

    #[test]
    fn skips_hidden() {
        let d = tempdir().unwrap();
        File::create(d.path().join(".hidden")).unwrap();
        File::create(d.path().join("visible")).unwrap();
        let out = list_dir(d.path().to_str().unwrap()).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "visible");
    }

    #[test]
    fn errors_on_nonexistent() {
        let res = list_dir("/definitely/not/here");
        assert!(matches!(res, Err(ListDirError::NotADirectory(_))));
    }

    #[test]
    fn serializes_is_dir_as_camelcase_isdir() {
        // Locks the FE/BE contract: TypeScript code reads `node.isDir`,
        // not `node.is_dir`. If this assertion fails, the file tree UI breaks.
        let n = FileNode {
            name: "x".into(),
            path: "/x".into(),
            is_dir: true,
        };
        let s = serde_json::to_string(&n).unwrap();
        assert!(s.contains("\"isDir\":true"), "expected camelCase isDir in {s}");
        assert!(!s.contains("is_dir"), "snake_case is_dir leaked: {s}");
    }
}
