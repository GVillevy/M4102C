use chat::actors::chat_client::*;
use chat::actors::{Listener, Writer};
use chat::*;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    client_name: String,
    server_addr: String,
    server_port: u16,
}
fn main() -> Result<()> {
    let options = Options::from_args();
    let chat_client = ChatClient::new(
        options.client_name,
        SocketAddr::new(IpAddr::from_str(&options.server_addr)?, options.server_port),
    )?;
    let chat_arc = Arc::new(chat_client);
    let chat_arc_clone = chat_arc.clone();
    let thread = thread::spawn(move || -> Result<()> {
        chat_arc_clone.listen()?;
        return Ok(());
    });
    chat_arc.write()?;
    thread.join().expect("Attente")?;
    return Ok(());
}