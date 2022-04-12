use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

struct WebSocket {
    write: String,
    read: String,
}

impl WebSocket {
    pub fn new() -> Self {
         let ws = WebSocket::open("ws://127.0.0.1:8080").unwrap();

        let (mut write, mut read) = ws.split();

        Self(write, read)
    }

    pub fn on_message(&mut self, f: fn(Message)) {
        spawn_local(async move {
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(msg) => {
                        f(msg);
                    }
                    Err(e) => {
                        log::error!("ws: {:?}", e)
                    }
                }
            }
            log::debug!("WebSocket Closed");
        });
    }
}
