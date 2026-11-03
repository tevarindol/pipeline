// SPDX-FileCopyrightText: 2026 Oleksandr Steblevets
// SPDX-License-Identifier: GPL-3.0-only

use crate::packets::Packet;
use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions,
    tokio::{Stream, prelude::*},
};
use std::{io, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::mpsc,
    task::JoinHandle,
};

pub struct Communicate {
    packets: mpsc::UnboundedReceiver<Packet>,
    listener: JoinHandle<()>,
}

fn name(role: &str) -> io::Result<interprocess::local_socket::Name<'static>> {
    format!("lab6-{role}").to_ns_name::<GenericNamespaced>()
}

pub fn create(role: &str) -> io::Result<Communicate> {
    let listener = ListenerOptions::new().name(name(role)?).create_tokio()?;
    let (sender, packets) = mpsc::unbounded_channel();
    let listener = tokio::spawn(async move {
        while let Ok(stream) = listener.accept().await {
            let mut stream = BufReader::new(stream);
            let mut message = String::new();
            if let Ok(Ok(size)) = tokio::time::timeout(
                Duration::from_secs(10),
                stream.read_line(&mut message),
            )
            .await
            {
                if size > 0 {
                    if let Ok(packet) = serde_json::from_str(&message) {
                        if sender.send(packet).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    });
    Ok(Communicate { packets, listener })
}

impl Communicate {
    pub async fn send_packet(&self, destination: &str, packet: &Packet) -> io::Result<()> {
        tokio::time::timeout(Duration::from_secs(10), async {
            let mut stream = Stream::connect(name(destination)?).await?;
            let mut message = serde_json::to_vec(packet).map_err(io::Error::other)?;
            message.push(b'\n');
            stream.write_all(&message).await?;
            stream.flush().await
        })
        .await?
    }

    pub async fn receive_packet(&mut self) -> io::Result<Packet> {
        self.packets.recv().await.ok_or_else(|| io::ErrorKind::BrokenPipe.into())
    }

    pub fn has_new_packets(&self) -> bool {
        !self.packets.is_empty()
    }
}

impl Drop for Communicate {
    fn drop(&mut self) {
        self.listener.abort();
    }
}
