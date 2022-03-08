use super::Result;

pub trait Listener {
    fn listen(&self) -> Result<()>;
}

pub trait Writer {
    fn write(&self) -> Result<()>;
    fn dispatch_line(&self, line: &str) -> Result<()>;
}

pub mod pair_chat_peer;