// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use crate::{App, Parameters, manager::ManagerCommand};
use common::validate;
use slint::{ComponentHandle, LogicalPosition};
use tokio::sync::mpsc;

pub fn configure(app: &App, dialog: &Parameters, commands: mpsc::UnboundedSender<ManagerCommand>) {
    app.window().on_close_requested(|| {
        let _ = slint::quit_event_loop();
        slint::CloseRequestResponse::HideWindow
    });
    app.window().set_position(LogicalPosition::new(20.0, 20.0));
    let dialog_weak = dialog.as_weak();
    app.on_open_parameters(move || {
        if let Some(dialog) = dialog_weak.upgrade() {
            let _ = dialog.show();
        }
    });

    let app_weak = app.as_weak();
    let dialog_weak = dialog.as_weak();
    dialog.on_execute(move |n, min, max| {
        let parsed = n
            .trim()
            .parse::<usize>()
            .ok()
            .zip(min.trim().parse::<f64>().ok())
            .zip(max.trim().parse::<f64>().ok())
            .map(|((n, min), max)| (n, min, max));
        let Some((n, min, max)) = parsed else {
            return;
        };
        if validate(min, max).is_err() {
            return;
        }
        if let Some(dialog) = dialog_weak.upgrade() {
            let _ = dialog.hide();
        }
        if let Some(app) = app_weak.upgrade() {
            app.set_busy(true);
            app.set_status("Виконання...".into());
        }
        if commands.send(ManagerCommand::Execute { n, min, max }).is_err() {
            if let Some(app) = app_weak.upgrade() {
                app.set_busy(false);
                app.set_status("".into());
            }
        }
    });
}
