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
        let mut user_id = 0;

        let (mut socket, _response) = connect(
    "ws://localhost:42142/ws/user"
        ).expect("Can't connect");

        let _ = socket.send(Message::Binary(vec![1].into()));

        loop {
            if let Ok(job) = job_rx.recv() {
                match job {
                    LocalUserUpdate::SendUserPosition(transform) => {
                        let mut data_sending = vec![8];
                        for byte in transform.position.0.to_be_bytes() {
                            data_sending.push(byte);
                        }
                        for byte in transform.position.1.to_be_bytes() {
                            data_sending.push(byte);
                        }
                        for byte in transform.position.2.to_be_bytes() {
                            data_sending.push(byte);
                        }
                        for byte in transform.rotation.0.to_be_bytes() {
                            data_sending.push(byte);
                        }
                        for byte in transform.rotation.1.to_be_bytes() {
                            data_sending.push(byte);
                        }
                        for byte in transform.rotation.2.to_be_bytes() {
                            data_sending.push(byte);
                        }

                        let _ = socket.send(Message::Binary(data_sending.into()));
                    }
                }
            }

            if let Ok(result) = socket.read() {
                let data = result.into_data();
                if data.len() > 0 {
                    match data[0] {
                        2 => {
                            user_id = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
                            println!("Received Free ID {}", user_id);
                        }
                        0 => {}
                        _ => { println!("Unknown Command ({})", data[0]); }
                    }
                }
            } else {
                println!("Socket has closed!")
            }
        }
    });
}