use std::sync::Arc;

use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use pkg::Pkg;
use tokio::sync::{mpsc::Receiver, Mutex};
use tokio::time::{timeout, Duration};

pub struct Ms {
    pub data: Arc<Mutex<Pkg>>,
    pub rx: Receiver<()>,
}

impl Ms {
    pub async fn send_message(&self, mut session: Session) {
        let locked_data = self.data.lock().await;

        // let json = json!(&locked_data.clone());
        let json = serde_json::to_string(&locked_data.clone()).unwrap();

        println!("Sending message length: {}", json.len());
        if let Err(e) = session.text(json).await {
            eprintln!("Failed to send message to client: {:?}", e);
        }
    }

    pub async fn handle_message(
        ms: Arc<Mutex<Ms>>,
        mut session: Session,
        mut msg_stream: MessageStream,
    ) {
        // 向前端发送消息
        let ms_clone = ms.clone();
        let session_clone = session.clone();
        tokio::spawn(async move {
            let ms_lock = ms_clone.lock().await;
            ms_lock.send_message(session_clone).await;
        });

        loop {
            let mut ms_lock = ms.lock().await;

            tokio::select! {
                Some(Ok(msg)) = msg_stream.next() =>{
                    match msg {
                        Message::Close(reason) => {
                            // 关闭连接
                            println!("client close with reason: {:?}", reason);
                        }
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

                // Some(_) = ms_lock.rx.recv()=> {
                //     println!("Got message");

                //     drop(ms_lock);

                //     let ms_lock = ms.lock().await;
                //
                //     ms_lock.send_message(session.clone()).await;
                // }
                result = timeout(Duration::from_millis(100),ms_lock.rx.recv())=>{
                    match result{
                        Ok(Some(_))=>{
                            drop(ms_lock);

                            let ms_lock = ms.lock().await;
                            ms_lock.send_message(session.clone()).await;
                        }
                        Ok(None)=>{
                            break;
                        }
                        Err(_)=>{
                            continue;
                        }
                    }
                }
            }
        }
    }
}
