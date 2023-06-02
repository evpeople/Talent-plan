/// Result ,kvs的Result类型

use thiserror::Error;
/// 派生出的KvsError们
#[derive(Error, Debug)]
pub enum KvsError{
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
    /// 默认的错误，开发用
    #[error("should not use")]
    DefaultError,
}

/// 对外暴露的Result，因为已经确定了一个类型，所以暴露在外的Result只有一个类型
pub type Result<T> = std::result::Result<T, KvsError>;