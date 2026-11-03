// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

pub mod clipboard;
pub mod communicate;
pub mod packets;

use anyhow::Result;
use std::io;

pub fn validate(min: f64, max: f64) -> Result<()> {
    if !min.is_finite() || !max.is_finite() || min > max || !(max - min).is_finite() {
        return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
    }
    Ok(())
}

pub fn format_values(values: &[f64]) -> String {
    values
        .chunks(3)
        .map(|row| {
            row.iter()
                .map(|value| format!("{value:>16.6e}"))
                .collect::<Vec<_>>()
                .join("  ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
