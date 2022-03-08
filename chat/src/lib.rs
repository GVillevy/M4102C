use serde::{Serialize,Deserialize};

pub mod actors;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;
pub const BUFFER_SIZE: usize = 2048;

#[derive(Serialize,Deserialize)]
pub struct IdMessage{
   pub name:String
}

#[derive(Serialize,Deserialize)]
pub struct ContentsMessage{
   pub sender : String,
   pub recipients: Vec<String>,
   pub payload: String
}