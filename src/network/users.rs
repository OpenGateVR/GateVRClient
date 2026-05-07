use tokio::io::{AsyncWriteExt, Result};
use tungstenite::{connect, Message};
use futures_util::{StreamExt, SinkExt};
use std::{sync::mpsc::Receiver, thread};

pub struct Transform {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32)
}

pub enum LocalUserUpdate {
    SendUserPosition(Transform)
}
pub enum UsersUpdate {
    SetUserPosition(Transform, u64)
}

pub fn start_user_handler(job_rx: Receiver<LocalUserUpdate>) {
    thread::spawn(move || {
        let (mut socket, _response) = connect(
    "ws://localhost:42142/ws/user"
        ).expect("Can't connect");

        let _ = socket.send(Message::text("test").into());

        loop {
            let msg = socket.read().expect("Error reading message");
            println!("Received: {}", msg);
        }
    });
}