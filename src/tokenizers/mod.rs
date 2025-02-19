pub mod builder;
pub mod encoding;
pub mod extensions;
pub mod implementation;
pub mod post_processor;
pub mod pre_tokenizer;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub type Offset = (usize, usize);
