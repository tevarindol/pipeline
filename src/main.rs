// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
//
// SPDX-License-Identifier: GPL-3.0-or-later

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    
    app.run()
}
