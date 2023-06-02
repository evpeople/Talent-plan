//! Talent plan 的前置课程,用于实现一个键值数据库,其中的键值均为是字符串
//!
//! 提供一个完善的,代码简单的键值数据库
//! 主要优势是,
//! - 简单
//! - [`有充分的教程`]
//!
//! [`有充分的教程`]:https://github.com/pingcap/talent-plan/tree/master/courses/rust

use std::collections::HashMap;
use std::path::Path;

/// KvStore , 键值数据库的实际结构体
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}
impl KvStore {
    /// 关联函数,用于创建一个新的KvStore
    ///  ```rust
    /// # use std::error::Error;
    /// # use kvs::KvStore;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let mut store = KvStore::new();

    ///     store.set("key1".to_owned(), "value1".to_owned());
    ///     store.set("key2".to_owned(), "value2".to_owned());
    ///     store.get("key1".to_owned());
    ///     store.remove("key1".to_owned());
    ///
    ///#    assert_eq!(store.get("key1".to_owned()), None);
    ///#   assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }
    /// Kvs 在文件路径打开log文件
    pub fn open(x: &Path){
        unimplemented!()
    }
    /// set 方法,在键值数据库中,设置一个值
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    /// get方法,在键值数据库中,得到一个Option
    pub fn get(&mut self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }
    /// remove方法,在键值数据库,删除一个值
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
