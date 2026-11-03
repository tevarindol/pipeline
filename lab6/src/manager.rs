// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use crate::App;
use anyhow::Result;
use common::{
    communicate::{self, Communicate},
    packets::{MANAGER, OBJECT2, OBJECT3, Packet},
};
use std::{io, time::Duration};
use tokio::{process::Command, sync::mpsc, time::Instant};

struct Manager {
    communicate: Communicate,
    next_id: u64,
}

impl Manager {
    fn next_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    async fn wait_for(&mut self, expected: u64) -> Result<()> {
        tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                if let Packet::Finished { id, success } = self.communicate.receive_packet().await? {
                    if id == expected {
                        return if success { Ok(()) } else { Err(io::Error::other("")) };
                    }
                }
            }
        }).await??;
        Ok(())
    }

    async fn ensure_worker(&mut self, name: &'static str) -> Result<()> {
        let id = self.next_id();
        let ping = Packet::Ping { id };
        match self.communicate.send_packet(name, &ping).await {
            Ok(()) => return self.wait_for(id).await,
            Err(error) if matches!(error.kind(), io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused) => {}
            Err(error) => return Err(error.into()),
        }

        let executable = std::env::current_exe()?
            .with_file_name(format!("{name}{}", std::env::consts::EXE_SUFFIX));
        let mut child = Command::new(executable).spawn()?;
        let deadline = Instant::now() + Duration::from_secs(10);
        while Instant::now() < deadline {
            if self.communicate.send_packet(name, &ping).await.is_ok() {
                return self.wait_for(id).await;
            }
            if child.try_wait()?.is_some() {
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted).into());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        let _ = child.kill().await;
        Err(io::Error::from(io::ErrorKind::TimedOut).into())
    }

    async fn execute(&mut self, n: usize, min: f64, max: f64) -> Result<()> {
        self.ensure_worker(OBJECT2).await?;
        let id = self.next_id();
        self.communicate.send_packet(OBJECT2, &Packet::Generate { id, n, min, max }).await?;
        self.wait_for(id).await?;

        self.ensure_worker(OBJECT3).await?;
        let id = self.next_id();
        self.communicate.send_packet(OBJECT3, &Packet::Sort { id }).await?;
        self.wait_for(id).await
    }
}

pub enum ManagerCommand {
    Execute { n: usize, min: f64, max: f64 },
    Shutdown,
}

pub async fn run(mut commands: mpsc::UnboundedReceiver<ManagerCommand>, app: slint::Weak<App>) {
    let Ok(communicate) = communicate::create(MANAGER) else { return };
    let mut manager = Manager { communicate, next_id: 0 };
    while let Some(command) = commands.recv().await {
        match command {
            ManagerCommand::Execute { n, min, max } => {
                let result = manager.execute(n, min, max).await;
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
        let _ = manager.communicate.send_packet(name, &Packet::Shutdown).await;
    }
}
