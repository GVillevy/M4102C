use std::net::{SocketAddr, UdpSocket};
use std::str;
use std::io::prelude::*;
use crate::actors::{Listener, Writer};
use crate::{Result, BUFFER_SIZE,ContentsMessage,IdMessage};

pub struct ChatClient{
   name:String,
   socket:UdpSocket,
   server_endpoint:SocketAddr
}

impl Listener for ChatClient{
   fn listen(&self) -> Result<()>{

      let mut buf = [0 ; BUFFER_SIZE];

      while let Ok((nb_bytes, _sender_endpoint)) = self.socket.recv_from(&mut buf) {
          let my_message = str::from_utf8(&buf[0..nb_bytes])?;
          println!("{:?}", my_message);
      }
      
      return Ok(())
   }

}

impl Writer for ChatClient{
   fn write(&self) -> Result<()>{

      let id_msg = IdMessage {
         name:self.name.clone(),
      };

      let buf = serde_json::to_vec(&id_msg)?;

      self.socket.send_to(&buf, self.server_endpoint)?;

      for line in std::io::stdin().lock().lines() {
         self.dispatch_line(&line?);
      }
      return Ok(());
   }

   fn dispatch_line(&self, line: &str) -> Result<()>{
      let mut line_elements = line.split(" : ");
      let recipients = line_elements.next().unwrap().split(", ").map(|x| x.to_string()).collect();
        
      match line_elements.next() {
          Some(payload) => {
              let contents_msg = ContentsMessage {
                  sender: self.name.clone(),
                  recipients,
                  payload: payload.to_string(),
              };
              let buf = serde_json::to_vec(&contents_msg)?;
              self.socket.send_to(&buf, self.server_endpoint)?;
          }
          None => println!("Wrongly typed message."),
      }   
      return Ok(());
  }
}

impl ChatClient{
   pub fn new(name: String, server_endpoint: SocketAddr) -> Result<Self> {
      return Ok(Self {
         name:name,
         socket: UdpSocket::bind("127.0.0.1:0")?,
         server_endpoint:server_endpoint
      });
  }
}