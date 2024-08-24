use std::sync::Arc;

use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use pkg::Pkg;
use tokio::sync::{mpsc::Receiver, Mutex};

use serde_json::json;
pub struct Ms {
    pub data: Arc<Mutex<Pkg>>,
    pub rx: Receiver<()>,
}

impl Ms {
    pub async fn send_message(&self, mut session: Session) {
        let locked_data = self.data.lock().await;

        let json = json!(&locked_data.clone());

        if let Err(e) = session.text(serde_json::to_string(&json).unwrap()).await {
            eprintln!("Failed to send message to client: {:?}", e);
        }
    }

    pub async fn handle_message(
        ms: Arc<Mutex<Ms>>,
        mut session: Session,
        mut msg_stream: MessageStream,
    ) {
        // 向前端发送消息
        // session.text("Hello").await.unwrap();

        let mut ms_lock = ms.lock().await;

        ms_lock.send_message(session.clone()).await;

        loop {
            tokio::select! {
                Some(Ok(msg)) = msg_stream.next() =>{
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

                Some(_) = ms_lock.rx.recv()=> {
                    println!("Got message");
                    // session.text("New data available").await.unwrap();
                    ms_lock.send_message(session.clone()).await;
                }
            }
        }
    }
}
