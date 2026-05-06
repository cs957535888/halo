use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;

use super::shell_detect::Shell;

pub struct PtySession {
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    child:  Arc<Mutex<Box<dyn Child + Send + Sync>>>,
}

impl PtySession {
    /// Spawn a PTY running `shell`, with initial size and cwd.
    pub fn spawn(shell: Shell, cols: u16, rows: u16, cwd: Option<&str>) -> Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })?;
        let mut cmd = CommandBuilder::new(shell.program);
        for a in shell.args { cmd.arg(a); }
        if let Some(d) = cwd { cmd.cwd(d); }
        let child = pair.slave.spawn_command(cmd)?;
        let writer = pair.master.take_writer()?;
        Ok(Self {
            master: Arc::new(Mutex::new(pair.master)),
            writer: Arc::new(Mutex::new(writer)),
            child:  Arc::new(Mutex::new(child)),
        })
    }

    /// Spawn a background thread that reads bytes and pushes them to `on_data`.
    pub fn start_reader<F: Fn(Vec<u8>) + Send + 'static>(&self, on_data: F) -> Result<()> {
        let mut reader = self.master.lock().try_clone_reader()?;
        thread::spawn(move || {
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0)  => break,
                    Ok(n)  => on_data(buf[..n].to_vec()),
                    Err(_) => break,
                }
            }
        });
        Ok(())
    }

    pub fn write(&self, bytes: &[u8]) -> Result<()> {
        self.writer.lock().write_all(bytes).map_err(|e| anyhow!(e))
    }

    pub fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        self.master.lock().resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })?;
        Ok(())
    }

    pub fn kill(&self) -> Result<()> {
        self.child.lock().kill()?;
        Ok(())
    }
}
