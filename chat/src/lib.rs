pub mod actors;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;
pub const BUFFER_SIZE: usize = 2048;
