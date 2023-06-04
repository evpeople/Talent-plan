use std::io::Error;
/// Result ,kvs的Result类型
use thiserror::Error;
/// 派生出的KvsError们
#[derive(Error, Debug)]
pub enum KvsError {
    // #[error("Invalid header (expected {expected:?}, got {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    // #[error("Missing attribute: {0}")]
    // MissingAttribute(String),
    /// 没有找到键
    #[error("key not find {0}")]
    KeyNotFound(String),
    /// SetError 在set 过程中出现的Error
    #[error("set error")]
    SetError,
    /// RmError 在remove 过程中出现的Error
    #[error("remove error")]
    RmError,
    /// IoError 为std::io::error From 添加的Error
    #[error("file io error")]
    IoError,
    /// SerdeError 为serde_json From 添加的Error
    #[error("serde io error")]
    SerdeError,
    /// 默认的错误，开发用
    #[error("should not use")]
    DefaultError,
}

impl From<std::io::Error> for KvsError {
    fn from(value: Error) -> Self {
        KvsError::IoError
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(value: serde_json::Error) -> Self {
        KvsError::SerdeError
    }
}
/// 对外暴露的Result，因为已经确定了一个类型，所以暴露在外的Result只有一个类型
pub type Result<T> = std::result::Result<T, KvsError>;
