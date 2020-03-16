// mod repository;
// mod handle;
use tokio::net::TcpListener;
use tokio::prelude::*;
use crate::handle;
use crate::repository;
use std::rc::Rc;
use std::sync::Arc;

pub trait Server :std::marker::Sync+std::marker::Send{
    fn start(&self,port: i16)-> Result<(), Box<dyn std::error::Error>> ;
}

pub struct TCPServer{
    h: Arc<dyn handle::Handler>
}


impl Server for TCPServer{
    fn start(&self,port: i16) -> Result<(), Box<dyn std::error::Error>>{
        self._start(port)
    }
}

pub fn new(h: Arc<dyn handle::Handler>) -> TCPServer{
    TCPServer{
        h:h
    }
}

impl TCPServer{
    #[tokio::main]
    async fn  _start(&self,port: i16)-> Result<(), Box<dyn std::error::Error>> {
        let mut listener = TcpListener::bind(format!("127.0.0.1:{}",port)).await?;
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
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    let index= match buf.iter().position(|&r| r == 3){
                        None => return,
                        Some(index)=>index
                    };
                    let name= match std::str::from_utf8(&buf[0..index]){
                        Ok(name)=> name,
                        Err(err)=>{println!("Err: {}",err);return;}
                    };
                    h.handle(name, &buf[index+1..n]);
                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    }
}

