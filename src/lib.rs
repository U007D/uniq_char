// Safety-critical application lints
pub mod error;
pub mod shared_consts;
mod uniq_char;

pub use error::{Error, Result};
pub use uniq_char::uniq_char;
