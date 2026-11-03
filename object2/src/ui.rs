// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use crate::App;
use anyhow::Result;
use slint::{ComponentHandle, LogicalPosition};

pub fn configure(app: &App) {
    app.window().set_position(LogicalPosition::new(20.0, 240.0));
}

pub fn show_values(app: &slint::Weak<App>, values: &[f64]) -> Result<()> {
    let display = common::format_values(values);
    app.upgrade_in_event_loop(move |app| app.set_values(display.into()))
        .map_err(|error| anyhow::anyhow!(error.to_string()))?;
    Ok(())
}

pub fn quit() {
    let _ = slint::invoke_from_event_loop(|| { let _ = slint::quit_event_loop(); });
}
