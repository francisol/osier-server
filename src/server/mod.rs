// mod repository;
// mod handle;
use crate::handle;
use crate::repository;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::prelude::*;
pub trait Server: std::marker::Sync + std::marker::Send {
    fn start(&self, port: i16) -> Result<(), Box<dyn std::error::Error>>;
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Reponse {
    code: i32,
    msg: crate::error::Error,
    data: handle::HanderResult,
}

pub struct TCPServer {
    h: Arc<dyn handle::Handler>,
}

impl Server for TCPServer {
    fn start(&self, port: i16) -> Result<(), Box<dyn std::error::Error>> {
        self._start(port)
    }
}

pub fn new(h: Arc<dyn handle::Handler>) -> TCPServer {
    TCPServer { h }
}

impl TCPServer {
    #[tokio::main]
    async fn _start(&self, port: i16) -> Result<(), Box<dyn std::error::Error>> {
        let mut listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        info!("Servr start on {}",port);
        loop {
            let (mut socket, _) = listener.accept().await?;
            let h = self.h.clone();
            tokio::spawn(async move {
                let mut buf = [0; 4096];
                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            error!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    let index = match buf.iter().position(|&r| r == 3) {
                        None => return,
                        Some(index) => index,
                    };
                    let name = match std::str::from_utf8(&buf[0..index]) {
                        Ok(name) => name,
                        Err(err) => {
                            error!("Err: {}", err);
                            return;
                        }
                    };
                    let r = match h.handle(name, &buf[index + 1..n]) {
                        Ok(r) => Reponse {
                            code: 0,
                            msg: crate::error::Error::OK,
                            data: r,
                        },
                        Err(e) => Reponse {
                            code: -100,
                            msg: e,
                            data: crate::handle::HanderResult::None,
                        },
                    };
                    let json = serde_json::to_vec(&r).unwrap();
                    let result = json.as_slice();
                    // Write the data back
                    if let Err(e) = socket.write_all(&result[..]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}
