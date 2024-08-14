use std::sync::mpsc::Receiver;

use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
pub struct Ms {
    rx: Receiver<()>,
}

impl Ms {
    pub fn send_message(&self) {}

    pub async fn handle_message(
        mut session: Session,
        mut msg_stream: MessageStream,
        rx: Receiver<()>,
    ) {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(msg) => {
                    println!("Got text: {msg}");
                    session.text("I got it!").await.unwrap();
                }
                _ => break,
            }
        }
        // let _ = session.close(None).await;

        if let Some(msg) = rx.recv().await {
            println!("Got message: {msg}")
        }
    }
}
