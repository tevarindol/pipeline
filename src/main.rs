// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
//
// SPDX-License-Identifier: GPL-3.0-or-later

use ipc_channel::ipc;

#[tokio::main]
async fn main() {
    let (tx, rx) = ipc::channel::<String>().unwrap();
    tx.send(String::from("Hello, World!")).unwrap();
    let message = rx.recv().unwrap();
    println!("{}", message);
}
