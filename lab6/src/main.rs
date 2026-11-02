// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

mod manager;
mod ui;

use anyhow::Result;
use slint::ComponentHandle;
use tokio::sync::mpsc;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new()?;
    let dialog = Parameters::new()?;
    let (commands, receiver) = mpsc::unbounded_channel();
    let manager = tokio::spawn(manager::run(receiver, app.as_weak()));

    ui::configure(&app, &dialog, commands.clone());
    let result = app.run();
    let _ = commands.send(manager::ManagerCommand::Shutdown);
    manager.await?;
    result?;
    Ok(())
}
