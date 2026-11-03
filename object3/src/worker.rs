// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use crate::{App, ui};
use anyhow::Result;
use common::{
    clipboard::Clipboard,
    communicate::Communicate,
    packets::{MANAGER, Packet},
};
use tokio::sync::oneshot;

pub async fn run(
    mut communicate: Communicate,
    app: slint::Weak<App>,
    mut stopped: oneshot::Receiver<()>,
) {
    let mut clipboard = Clipboard::default();
    loop {
        let packet = tokio::select! {
            packet = communicate.receive_packet() => match packet {
                Ok(packet) => packet,
                Err(_) => break,
            },
            _ = &mut stopped => return,
        };
        let (id, result) = match packet {
            Packet::Ping { id } => (id, Ok(())),
            Packet::Sort { id } => {
                let result: Result<()> = (|| {
                    let mut values = clipboard.read_vector()?;
                    values.sort_by(f64::total_cmp);
                    ui::show_values(&app, &values)
                })();
                (id, result)
            }
            Packet::Shutdown => break,
            _ => continue,
        };
        let _ = communicate.send_packet(MANAGER, &Packet::Finished { id, success: result.is_ok() }).await;
    }
    ui::quit();
}
