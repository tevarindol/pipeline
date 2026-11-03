// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

mod ui;
mod worker;

use anyhow::Result;
use common::{communicate, packets::OBJECT2};
use slint::ComponentHandle;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new()?;
    ui::configure(&app);
    let communicate = communicate::create(OBJECT2)?;
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(worker::run(communicate, app.as_weak(), stopped));

    let result = app.run();
    let _ = stop.send(());
    task.await?;
    result?;
    Ok(())
}
