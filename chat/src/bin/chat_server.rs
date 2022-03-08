use chat::actors::chat_server::ChatServer;
use chat::actors::Server;
use chat::*;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options {
    server_port: u16,
}
fn main() -> Result<()> {
    let options = Options::from_args();
    let chat_server = ChatServer::new(options.server_port)?;
    chat_server.serve()?;
    return Ok(());
}