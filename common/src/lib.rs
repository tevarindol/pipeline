// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::{Result, anyhow};
use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions,
    tokio::{Listener, Stream, prelude::*},
};
use serde::{Deserialize, Serialize};
use std::io;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub const OBJECT2: &str = "object2";
pub const OBJECT3: &str = "object3";
pub type Connection = Stream;

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Generate { n: usize, min: f64, max: f64 },
    Sort,
    Shutdown,
}

fn name(role: &str) -> io::Result<interprocess::local_socket::Name<'static>> {
    format!("lab6-{role}").to_ns_name::<GenericNamespaced>()
}

pub fn listener(role: &str) -> Result<Listener> {
    ListenerOptions::new()
        .name(name(role)?)
        .create_tokio()
        .map_err(Into::into)
}

pub async fn connect(role: &str) -> io::Result<Stream> {
    Stream::connect(name(role)?).await
}

pub async fn send(stream: Stream, request: Request) -> Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        let mut stream = BufReader::new(stream);
        let mut message = serde_json::to_string(&request)?;
        message.push('\n');
        stream.get_mut().write_all(message.as_bytes()).await?;
        let mut response = String::new();
        if stream.read_line(&mut response).await? == 0 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof).into());
        }
        let result: std::result::Result<(), String> = serde_json::from_str(&response)?;
        result.map_err(|error| anyhow!(error))
    })
    .await?
}

pub async fn listen(listener: Listener, mut handle: impl FnMut(Request) -> Result<()>) -> Result<()> {
    loop {
        let stream = listener.accept().await?;
        let mut stream = BufReader::new(stream);
        let mut message = String::new();
        match tokio::time::timeout(Duration::from_secs(10), stream.read_line(&mut message)).await {
            Ok(Ok(0)) => continue,
            Ok(Ok(_)) => {}
            Ok(Err(_)) => continue,
            Err(_) => continue,
        }
        let request: Request = match serde_json::from_str(&message) {
            Ok(request) => request,
            Err(_) => continue,
        };
        let shutdown = matches!(request, Request::Shutdown);
        let response = handle(request).map_err(|error| format!("{error:#}"));
        let mut response = serde_json::to_string(&response)?;
        response.push('\n');
        let _ = stream.get_mut().write_all(response.as_bytes()).await;
        if shutdown {
            return Ok(());
        }
    }
}

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

pub fn encode_values(values: &[f64]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn decode_values(text: &str) -> Result<Vec<f64>> {
    let values: Vec<f64> = text
        .split_whitespace()
        .map(|s| s.parse::<f64>())
        .collect::<std::result::Result<_, _>>()?;
    Ok(values)
}
