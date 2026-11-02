// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use common::{OBJECT3, Request};
use anyhow::Result;
use slint::{ComponentHandle, LogicalPosition};

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new()?;
    app.window()
        .set_position(LogicalPosition::new(540.0, 240.0));
    let weak = app.as_weak();
    let listener = common::listener(OBJECT3)?;
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let task = tokio::spawn(async move {
        let mut clipboard: Option<arboard::Clipboard> = None;
        let listen = common::listen(listener, |request| {
            match request {
                Request::Shutdown => Ok(()),
                Request::Sort => (|| {
                    if clipboard.is_none() {
                        clipboard = Some(arboard::Clipboard::new()?);
                    }
                    let text = clipboard
                        .as_mut()
                        .unwrap()
                        .get_text()?;
                    let mut values = common::decode_values(&text)?;
                    values.sort_by(f64::total_cmp);
                    let display = common::format_values(&values);
                    weak.upgrade_in_event_loop(move |app| app.set_values(display.into()))
                        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
                    Ok(())
                })(),
                _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidInput).into()),
            }
        });
        tokio::select! {
            _ = listen => {
                let _ = slint::invoke_from_event_loop(|| { let _ = slint::quit_event_loop(); });
            }
            _ = stopped => {}
        }
    });
    let result = app.run();
    let _ = stop.send(());
    task.await?;
    result?;
    Ok(())
}
