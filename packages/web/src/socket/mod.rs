use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use pkg::package::{Package, Pkg};
use tokio::time::{timeout, Duration};

pub struct Ms {}

impl Ms {
    pub async fn send_message(data: Pkg, session: &mut Session) {
        // let locked_data = data.get_pkg().await;

        // let json = json!(&locked_data.clone());
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
            // channel 通信消息
            let mut rx = ms.receiver.lock().await;

            tokio::select! {
                result = timeout(Duration::from_millis(100),rx.recv())=>{
                    match result{
                        Ok(Some(_)) => {
                            println!("rx recv msg");
                            let data_clone = ms.get_pkg().await;
                            Ms::send_message(data_clone, &mut session).await;
                        }
                        _ => {
                            continue;
                        }
                    }
                }
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

            }
        }

        let _ = session.close(None).await;
    }
}
