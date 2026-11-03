// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;

#[derive(Default)]
pub struct Clipboard {
    clipboard: Option<arboard::Clipboard>,
}

impl Clipboard {
    pub fn write_vector(&mut self, values: &[f64]) -> Result<()> {
        if self.clipboard.is_none() {
            self.clipboard = Some(arboard::Clipboard::new()?);
        }
        let text = values.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n");
        self.clipboard.as_mut().unwrap().set_text(text)?;
        Ok(())
    }

    pub fn read_vector(&mut self) -> Result<Vec<f64>> {
        if self.clipboard.is_none() {
            self.clipboard = Some(arboard::Clipboard::new()?);
        }
        let text = self.clipboard.as_mut().unwrap().get_text()?;
        Ok(text.split_whitespace().map(str::parse).collect::<Result<Vec<_>, _>>()?)
    }
}
