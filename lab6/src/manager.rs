// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use common::{Connection, OBJECT2, OBJECT3, Request, connect, send};
use crate::App;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::Instant;

async fn ensure_worker(name: &'static str) -> Result<Connection> {
    match connect(name).await {
        Ok(sender) => return Ok(sender),
        Err(error) if matches!(error.kind(), std::io::ErrorKind::NotFound | std::io::ErrorKind::ConnectionRefused) => {},
        Err(error) => return Err(error.into()),
    }
    let executable = std::env::current_exe()?
        .with_file_name(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let mut child = Command::new(&executable).spawn()?;
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if let Ok(sender) = connect(name).await {
            return Ok(sender);
        }
        if child.try_wait()?.is_some() {
            return Err(std::io::Error::from(std::io::ErrorKind::ConnectionAborted).into());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let _ = child.kill().await;
    Err(std::io::Error::from(std::io::ErrorKind::TimedOut).into())
}

async fn execute(n: usize, min: f64, max: f64) -> Result<()> {
    send(ensure_worker(OBJECT2).await?, Request::Generate { n, min, max }).await?;
    send(ensure_worker(OBJECT3).await?, Request::Sort).await?;
    Ok(())
}

pub enum ManagerCommand {
    Execute { n: usize, min: f64, max: f64 },
    Shutdown,
}

pub async fn run(mut commands: mpsc::UnboundedReceiver<ManagerCommand>, app: slint::Weak<App>) {
    while let Some(command) = commands.recv().await {
        match command {
            ManagerCommand::Execute { n, min, max } => {
                let result = execute(n, min, max).await;
                let _ = app.upgrade_in_event_loop(move |app| {
                    app.set_busy(false);
                    app.set_status(if result.is_ok() {
                        "Готово. Можна виконати завдання повторно.".into()
                    } else {
                        "".into()
                    });
                });
            }
            ManagerCommand::Shutdown => break,
        }
    }
    for name in [OBJECT2, OBJECT3] {
        if let Ok(channel) = connect(name).await {
            let _ = send(channel, Request::Shutdown).await;
        }
    }
}
