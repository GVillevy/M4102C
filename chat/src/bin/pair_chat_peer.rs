use structopt::StructOpt;
use chat::actors::pair_chat_peer::*;
use chat::Result;
use chat::actors::{Listener, Writer};
use std::net::SocketAddr;
use std::thread;
use std::sync::Arc;

#[derive(StructOpt)]
struct Options{
    port_ecoute: u16,
    ip_pair: String,
    port_pair: u16,
}

fn main() -> Result<()>{
    let my_options = Options::from_args();
    let socket_pair = format!("{}:{}", my_options.ip_pair, my_options.port_pair);
    let chat = PairChatPeer::new(my_options.port_ecoute, socket_pair.parse::<SocketAddr>()?)?;
    let my_arc = Arc::new(chat);
    let my_arc_clone = my_arc.clone();
    let my_thread = thread::spawn(move || {
        my_arc_clone.write();
        
    });
    my_arc.listen();
    return Ok(());
}