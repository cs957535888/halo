use crate::workspace::file_tree::{list_dir as inner_list, FileNode};

#[tauri::command]
pub fn list_dir(path: String) -> Result<Vec<FileNode>, String> {
    inner_list(&path).map_err(|e| e.to_string())
}
