//! Detect a sensible default shell binary for the current OS.
//! Order:
//!   Windows:  $COMSPEC -> pwsh.exe -> powershell.exe -> cmd.exe
//!   Unix:     $SHELL   -> /bin/zsh -> /bin/bash      -> /bin/sh

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Shell {
    pub program: String,
    pub args: Vec<String>,
}

pub fn detect_default() -> Shell {
    if cfg!(target_os = "windows") {
        if let Ok(c) = std::env::var("COMSPEC") { return Shell { program: c, args: vec![] }; }
        for cand in ["pwsh.exe", "powershell.exe", "cmd.exe"] {
            if which(cand).is_some() { return Shell { program: cand.to_string(), args: vec![] }; }
        }
        Shell { program: "cmd.exe".to_string(), args: vec![] }
    } else {
        if let Ok(s) = std::env::var("SHELL") { return Shell { program: s, args: vec!["-l".into()] }; }
        for cand in ["/bin/zsh", "/bin/bash", "/bin/sh"] {
            if PathBuf::from(cand).exists() { return Shell { program: cand.to_string(), args: vec!["-l".into()] }; }
        }
        Shell { program: "/bin/sh".to_string(), args: vec![] }
    }
}

#[cfg(target_os = "windows")]
fn which(prog: &str) -> Option<PathBuf> {
    let path = std::env::var("PATH").ok()?;
    for dir in path.split(';') {
        let p = PathBuf::from(dir).join(prog);
        if p.exists() { return Some(p); }
    }
    None
}
#[cfg(not(target_os = "windows"))]
fn which(_prog: &str) -> Option<PathBuf> { None }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detect_returns_nonempty_program() {
        let s = detect_default();
        assert!(!s.program.is_empty());
    }
}
