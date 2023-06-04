#![deny(missing_docs)]
//! A simple key/value store.

pub use error::KvsError;
pub use error::Result;
pub use kv::KvStore;
mod error;
mod kv;
