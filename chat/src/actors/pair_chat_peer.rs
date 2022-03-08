use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::io::prelude::*;
use crate::actors::{Listener, Writer};
use crate::{Result, BUFFER_SIZE};

pub struct PairChatPeer {
    peer_endpoint: SocketAddr,
    socket: UdpSocket,
}

impl PairChatPeer {
    pub fn new(port: u16, peer_endpoint: SocketAddr) -> Result<Self> {
        return Ok(Self {
            peer_endpoint,
            socket: UdpSocket::bind(format!("127.0.0.1:{}", port))?,
        });
    }
}

impl Listener for PairChatPeer {
    fn listen(&self) -> Result<()>{

        let mut buf = [0 ; BUFFER_SIZE];

        while let Ok((nb_bytes, _sender_endpoint)) = self.socket.recv_from(&mut buf) {
            let my_message = str::from_utf8(&buf[0..nb_bytes])?;
            println!("{:?}", my_message);
        }
        
        return Ok(())
    }
}

impl Writer for PairChatPeer {  
    fn dispatch_line(&self, line: &str) -> Result<()>{
        self.socket.send_to(line.as_bytes(), self.peer_endpoint)?;
        return Ok(());
    }
    
    fn write(&self) -> Result<()>{
        for line in std::io::stdin().lock().lines() {
            self.dispatch_line(&line.unwrap());
        }
        return Ok(())
    }
}