use super::*;
use crate::*;
use std::{cell::RefCell, collections::HashMap, net::UdpSocket};
pub struct ChatServer {
    clients: RefCell<HashMap<String, SocketAddr>>,
    socket:UdpSocket
}


impl Server for ChatServer{
   fn handle_message (&self, sender_address:SocketAddr, received_message: &[u8]) -> Result<()>{
      match serde_json::from_slice::<IdMessage>(&received_message) {
         Ok(msg) => {
             self.clients.borrow_mut().insert(msg.name, sender_address);
         }
         Err(_err) => match serde_json::from_slice::<ContentsMessage>(&received_message) {
             Ok(msg) => {
                 for recipient in &msg.recipients {
                     match self.clients.borrow_mut().get(recipient) {
                         Some(addr) => {
                             let buf = serde_json::to_vec(&msg)?;
                             self.socket.send_to(&buf, addr)?;
                         }
                         None => {
                             self.socket.send_to(
                                 format!("Unknown recipient : \"{}\"", recipient).as_bytes(),
                                 sender_address,
                             )?;
                         }
                     }
                 }
             }
             Err(_err) => println!("Non-supported message received."),
         },
     }
      return Ok(());
   }

   fn serve(&self) -> Result<()>{
      let mut buf = [0; BUFFER_SIZE];
      while let Ok((nb_bytes, sender_endpoint)) = self.socket.recv_from(&mut buf) {
          self.handle_message(sender_endpoint, &buf[0..nb_bytes])?;
      }
      return Ok(());
   }
}

impl ChatServer {
   pub fn new(port: u16) -> Result<Self> {
       return Ok(Self {
           clients: RefCell::new(HashMap::new()),
           socket: UdpSocket::bind(format!("127.0.0.1:{}", port))?,
       });
   }
}