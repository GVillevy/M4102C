use super::Result;
use std::net::SocketAddr;
pub trait Listener {
    fn listen(&self) -> Result<()>;
}

pub trait Writer {
    fn write(&self) -> Result<()>;
    fn dispatch_line(&self, line: &str) -> Result<()>;
}

pub mod pair_chat_peer;

pub trait Server{
    fn serve(&self) -> Result<()>;
    fn handle_message (&self, sender_address:SocketAddr, received_message: &[u8]) -> Result<()>;
}

pub mod chat_client;
pub mod chat_server;