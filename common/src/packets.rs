// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};

pub const MANAGER: &str = "manager";
pub const OBJECT2: &str = "object2";
pub const OBJECT3: &str = "object3";

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    Ping { id: u64 },
    Generate { id: u64, n: usize, min: f64, max: f64 },
    Sort { id: u64 },
    Shutdown,
    Finished { id: u64, success: bool },
}
