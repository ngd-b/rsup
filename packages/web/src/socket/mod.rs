use std::{collections::HashMap, sync::Arc};

use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use pkg::package::{Package, Pkg};
use tokio::sync::Mutex;

/// 随即创建的连接id
pub type ConnId = usize;

#[derive(Clone)]
pub struct Ms {
    pub package: Package,
    pub connectors: Arc<Mutex<HashMap<ConnId, Session>>>,
}

impl Ms {
    pub fn new(package: Package) -> Self {
        Ms {
            package,
            connectors: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    /// 单发消息
    /// 想发就发
    pub async fn send_message(data: Pkg, session: &mut Session) {
        let json = serde_json::to_string(&data.clone()).unwrap();

        println!("Sending message length: {}", json.len());
        if let Err(e) = session.text(json).await {
            eprintln!("Failed to send message to client: {:?}", e);
        }
    }

    /// websocket服务消息
    ///
    /// - 处理socket消息
    ///
    /// - 处理channel通道消息
    pub async fn handle_message(ms: Package, mut session: Session, mut msg_stream: MessageStream) {
        // 向前端发送消息
        // let ms_clone = ms.clone();

        let data_clone = ms.get_pkg().await;

        Ms::send_message(data_clone, &mut session).await;

        loop {
            tokio::select! {
                Some(Ok(msg)) = msg_stream.next() =>{
                    match msg {
                        Message::Close(reason) => {
                            // 关闭连接
                            println!("client close with reason: {:?}", reason);
                            break;
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

            }
        }

        let _ = session.close(None).await;
    }

    /// 接收channel 通道消息
    ///
    /// 这里是群发消息
    /// 接收到channel通道消息后，群发所有连接的客户端
    pub async fn handle_receiver_msg(&mut self) {
        loop {
            let mut rx: tokio::sync::MutexGuard<'_, tokio::sync::mpsc::Receiver<()>> =
                self.package.receiver.lock().await;

            if let Some(_) = rx.recv().await {
                let data_clone = self.package.get_pkg().await;

                let json = serde_json::to_string(&data_clone).unwrap();

                let connectors: Vec<_> = self
                    .connectors
                    .lock()
                    .await
                    .iter_mut()
                    .map(|(cond_id, session)| (*cond_id, session.clone()))
                    .collect();
                println!(
                    "Will send message to every client! the total of client is {}",
                    connectors.len()
                );
                for (conn_id, mut session) in connectors {
                    println!("Sending message to conn_id {:?}", conn_id);

                    let json_clone = json.clone();

                    tokio::spawn(async move {
                        if let Err(e) = session.text(json_clone).await {
                            eprintln!(
                                "Failed to send message to client {}. error:{:?}",
                                conn_id, e
                            );
                        }
                    });
                }
            }
        }
    }
}
