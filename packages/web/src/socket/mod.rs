use actix_ws::{Message, MessageStream, Session};
use futures_util::StreamExt;
use pkg::package::Package;
use tokio::time::{timeout, Duration};

pub struct Ms {}

impl Ms {
    pub async fn send_message(data: Package, mut session: Session) {
        let locked_data = data.get_pkg().await;

        // let json = json!(&locked_data.clone());
        let json = serde_json::to_string(&locked_data.clone()).unwrap();

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
        let ms_clone = ms.clone();
        let session_clone = session.clone();

        tokio::spawn(async move {
            Ms::send_message(ms_clone, session_clone).await;
        });

        loop {
            let mut rx = ms.receiver.lock().await;
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

                result = timeout(Duration::from_millis(100),rx.recv())=>{
                    match result{
                        Ok(Some(_))=>{
                            drop(rx);

                            let ms_lock = ms.clone();
                            Ms::send_message(ms_lock,session.clone()).await;
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
