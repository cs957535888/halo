use crate::pty::{
    manager::PtyManager,
    session::PtySession,
    shell_detect::detect_default,
};
use crate::workspace::file_tree::{list_dir as inner_list, FileNode};

use tauri::{Emitter, State};

#[tauri::command]
pub fn list_dir(path: String) -> Result<Vec<FileNode>, String> {
    inner_list(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pty_spawn(
    cols: u16,
    rows: u16,
    cwd: Option<String>,
    state: State<'_, PtyManager>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let shell = detect_default();
    let sess = PtySession::spawn(shell, cols, rows, cwd.as_deref()).map_err(|e| e.to_string())?;
    let id = state.insert(sess);
    let session = state.get(&id).expect("just inserted");
    let id_for_event = id.clone();
    session
        .start_reader(move |bytes| {
            let text = String::from_utf8_lossy(&bytes).to_string();
            let _ = app.emit(&format!("pty://{id_for_event}"), text);
        })
        .map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
pub fn pty_write(id: String, data: String, state: State<'_, PtyManager>) -> Result<(), String> {
    let s = state.get(&id).ok_or_else(|| format!("no session: {id}"))?;
    s.write(data.as_bytes()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pty_resize(id: String, cols: u16, rows: u16, state: State<'_, PtyManager>) -> Result<(), String> {
    let s = state.get(&id).ok_or_else(|| format!("no session: {id}"))?;
    s.resize(cols, rows).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pty_kill(id: String, state: State<'_, PtyManager>) -> Result<(), String> {
    let s = state.remove(&id).ok_or_else(|| format!("no session: {id}"))?;
    s.kill().map_err(|e| e.to_string())
}
