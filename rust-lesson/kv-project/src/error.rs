/// Result ,kvs的Result类型

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError{
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

/// 对外暴露的Result，因为已经确定了一个类型，所以暴露在外的Result只有一个类型
pub type Result<T> = std::result::Result<T, KvsError>;