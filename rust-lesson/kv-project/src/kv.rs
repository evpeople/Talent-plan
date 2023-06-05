//! Talent plan 的前置课程,用于实现一个键值数据库,其中的键值均为是字符串
//!
//! 提供一个完善的,代码简单的键值数据库
//! 主要优势是,
//! - 简单
//! - [`有充分的教程`]
//!
//! [`有充分的教程`]:https://github.com/pingcap/talent-plan/tree/master/courses/rust

use crate::error;

use crate::KvsError::{KeyNotFound, RmError, SetError};
use error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// KvStore , 键值数据库的实际结构体
pub struct KvStore {
    old_data_files: Vec<File>,
    active_file: File,
    active_file_index: usize,
    map: HashMap<String, BitCaskValue>,
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore {
            active_file_index: 0,
            old_data_files: vec![],
            active_file: (File::open(temp_dir())).unwrap(),
            map: HashMap::new(),
        }
    }
}
#[derive(Deserialize, Serialize)]
enum Commands {
    Get {
        key: String,
    },
    Set {
        t_stamp: i64,
        // #[serde(skip_deserializing)]
        k_size: usize,
        v_size: usize,
        key: String,
        // #[serde(skip_deserializing)]
        value: String,
    },
    Rm {
        t_stamp: i64,
        k_size: usize,
        v_size: usize,
        key: String,
        value: String,
    },
}

impl Commands {
    fn to_string(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
struct BitCaskValue {
    file_id: usize,
    v_size: usize,
    value_pos: usize,
    t_stamp: i64,
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
        Default::default()
    }
    /// Kvs 在文件路径打开log文件
    pub fn open(x: &Path) -> Result<KvStore> {
        let mut map = HashMap::new();
        let mut read_func = |f: &File, file_id: usize| -> Result<Option<String>> {
            let reader = BufReader::new(f);
            for (index, line) in reader.lines().enumerate() {
                let value: Commands = serde_json::from_str(line?.as_str())?;
                let bit_cask_value = if let Commands::Set {
                    t_stamp,
                    k_size: _,
                    v_size,
                    key,
                    value: _,
                } = value
                {
                    Some((
                        key,
                        BitCaskValue {
                            file_id,
                            v_size,
                            value_pos: index,
                            t_stamp,
                        },
                    ))
                } else {
                    None
                };
                let (key, value) = bit_cask_value.unwrap();
                map.insert(key, value);
            }
            Ok(Some("ok".to_string()))
        };
        let mut files = vec![];
        let active_file = if let Some(entry) = fs::read_dir(x)?.next() {
            File::open(entry?.path())?
        } else {
            unreachable!();
        };
        read_func(&active_file, 0)?;

        for (index, entry_result) in fs::read_dir(x)?.enumerate() {
            let entry = entry_result?;
            if let Ok(file) = File::open(entry.path()) {
                read_func(&file, index)?;
                files.push(file);
            }
        }
        Ok(KvStore {
            old_data_files: files,
            active_file,
            map,
            active_file_index: 0,
        })
    }
    /// set 方法,在键值数据库中,设置一个值
    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
        use chrono::Utc;
        let stamp = Utc::now().timestamp();
       let _= self.active_file.write(
            Commands::Set {
                t_stamp: stamp,
                k_size: key.len(),
                v_size: value.len(),
                key: key.clone(),
                value: value.clone(),
            }
            .to_string()
            .map_err(|_| SetError)?
            .as_bytes(),
        )?;
        let bcv = BitCaskValue {
            file_id: 0,
            v_size: value.len(),
            value_pos: self.active_file_index,
            t_stamp: stamp,
        };
        self.active_file_index += 1;
        self.map.insert(key, bcv);
        Ok(None)
    }
    /// get方法,在键值数据库中,得到一个Option
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.map
            .get(key.as_str())
            .ok_or(KeyNotFound(key))
            .and_then(|bcv| {
                let file =if bcv.file_id == 0 {
                     &self.active_file
                } else {
                     self.old_data_files.get(bcv.file_id).unwrap()
                };
                let reader = BufReader::new(file);
                let kv: Commands =
                    serde_json::from_str(reader.lines().nth(bcv.value_pos- 1).unwrap()?.as_str())?;
                let value = match kv {
                    Commands::Set {
                        t_stamp: _,
                        k_size: _,
                        v_size: _,
                        key: _,
                        value,
                    } => {

                            value

                    }
                    _ => "Key not find".to_string(),
                };

                Ok(Some(value))
            })
    }
    /// remove方法,在键值数据库,删除一个值
    ///
    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        match self.map.get(key.as_str()) {
            Some(_t) => {
                use chrono::Utc;
                let stamp = Utc::now().timestamp();
                let _=self.active_file.write(
                    Commands::Set {
                        t_stamp: stamp,
                        k_size: key.len(),
                        v_size: 0,
                        key: key.clone(),
                        value: "".to_string(),
                    }
                    .to_string()
                    .map_err(|_| RmError)?
                    .as_bytes(),
                )?;
                self.active_file_index += 1;
                self.map.remove(key.as_str());
                Ok(None)
            }
            None => Ok(None),
        }
    }
}
