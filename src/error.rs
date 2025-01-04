use derive_more::derive::{Display, Error};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Display, Error)]
pub enum Error {}
