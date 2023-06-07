//! Talent plan 的前置课程,用于实现一个键值数据库,其中的键值均为是字符串
//!
//! 提供一个完善的,代码简单的键值数据库
//! 主要优势是,
//! - 简单
//! - [`有充分的教程`]
//!
//! [`有充分的教程`]:https://github.com/pingcap/talent-plan/tree/master/courses/rust

use std::cmp::Ordering;
use crate::error;

use crate::KvsError::{DefaultError, RmError, SetError};
use error::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use std::env::temp_dir;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};
/// KvStore , 键值数据库的实际结构体
pub struct KvStore {
    old_data_files: Vec<File>,
    active_file: File,
    active_file_index: usize,
    file_path: PathBuf,
    map: HashMap<String, BitCaskValue>,
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore {
            active_file_index: 0,
            old_data_files: vec![],
            active_file: (File::open(temp_dir())).unwrap(),
            map: HashMap::new(),
            file_path: PathBuf::default(),
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
        k_size: usize,
        v_size: usize,
        key: String,
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
    fn to_string(&self) -> Result<String> {
        Ok(format!("{}\n", serde_json::to_string(self)?))
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
        let mut a_index = 0;
        let mut read_func = |f: &File, file_id: usize| -> Result<Option<String>> {
            let reader = BufReader::new(f);
            for (index, line) in reader.lines().enumerate() {
                if file_id == 0 {
                    a_index += 1;
                }
                let line = match line {
                    Ok(line) => line,
                    Err(_) => break,
                };
                let value: Commands = serde_json::from_str(line.as_str())?;
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
        let mut unsort_files: Vec<_> = fs::read_dir(x)?.filter_map(|entry| entry.ok()).collect();
        unsort_files.sort_by_key(|path| path.file_name());
        let mut it = unsort_files.iter();
        let mut binding = OpenOptions::new();
        let file_open_option = binding.read(true).append(true);
        let active_file = if let Some(entry) = it.next() {
            file_open_option.open(entry.path())?
        } else {
            file_open_option.create(true).open(x.join("0"))?
        };
        read_func(&active_file, 0)?;
        let mut files = vec![];
        for (index, entry_result) in it.enumerate() {
            let entry = entry_result;
            if let Ok(file) = file_open_option.open(entry.path()) {
                read_func(&file, index)?;
                files.push(file);
            }
        }
        Ok(KvStore {
            old_data_files: files,
            active_file,
            map,
            active_file_index: a_index,
            file_path: x.to_path_buf(),
        })
    }
    fn check_and_change_active_file(&mut self) {
        if self.active_file.metadata().unwrap().len() < 1000 {
            return;
        }
        let old_file = self.active_file.try_clone().unwrap();
        let file_name = (self.old_data_files.len() + 1).to_string();
        let new_file_path = self.file_path.join(file_name);
        let new_active_file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(new_file_path);
        self.old_data_files.push(old_file);
        self.active_file = new_active_file.unwrap();
        self.active_file_index = 0;
    }
    fn write_to_file(&mut self,map:HashMap<String,Commands>){
        let path=self.file_path.join("new");
        let mut file =File::create(path).unwrap();
        let mut entries =vec!();
        map.iter().for_each(|kv|{
            entries.push(kv.1)
        });
        entries.sort_by(|a,b|{
            match (a,b) {
                (Commands::Set {t_stamp:ts1,..},Commands::Set {t_stamp:ts2,..})=>{
                    ts1.cmp(ts2)
                },
                (_,_)=>{
                    Ordering::Equal
                }
            }
        });
        entries.iter().for_each(
            |e|{
                file.write(e.to_string().unwrap().as_bytes()).unwrap();
            }
        )


    }
    fn renew_file(&mut self){
        let p=self.file_path.clone();
        for i in 1..self.old_data_files.len() {
           fs::remove_file(p.join(i.to_string())).unwrap()
        }
        fs::rename(p.join("new".to_string()),p.join("1".to_string()));
    }
    fn compress(&mut self) -> HashMap<String, Commands> {
        let mut new_kv = HashMap::new();
        let _ = self.old_data_files.iter().for_each(|f| {
            let mut nf = f.try_clone().unwrap();
            nf.rewind().unwrap();
            let reader = BufReader::new(nf);
            reader.lines().for_each(|line| {
                let nl = line.unwrap();
                let kv: Commands = serde_json::from_str(nl.as_str()).unwrap();
                let kv = match kv {
                    Commands::Set {
                        t_stamp: ts,
                        k_size: ks,
                        v_size: vs,
                        key ,
                        value,
                    } => {
                        if !value.is_empty() {
                            Some((key.clone(),Commands::Set {
                                t_stamp: ts,
                                k_size: ks,
                                v_size: vs,
                                key,
                                value,
                            }))
                        }else {
                            None
                        }
                    }
                    _ => None,
                };
                let (k,v)=kv.unwrap();
                new_kv.insert(k,v);
            });
        });
        return new_kv
    }
    fn total_compress(&mut self){
        self.check_and_change_active_file();
      let kv=  self.compress();
        self.write_to_file(kv);
        self.renew_file();
    }
    /// set 方法,在键值数据库中,设置一个值
    pub fn set(&mut self, key: String, value: String) -> Result<Option<String>> {
        self.total_compress();
        use chrono::Utc;
        let stamp = Utc::now().timestamp();
        let _ = self.active_file.write(
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
        let file_id = self.old_data_files.len();
        let bcv = BitCaskValue {
            file_id,
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
        // let v=self.map.get(key.as_str()).ok_or(KeyNotFound(key));
        let res = self.map.get(key.as_str()).map(|bcv| {
            let mut file = if bcv.file_id == self.old_data_files.len() {
                self.active_file.try_clone().ok()?
            } else {
                self.old_data_files
                    .get(bcv.file_id - 1)
                    .unwrap()
                    .try_clone()
                    .ok()?
            };
            file.rewind().ok()?;
            let reader = BufReader::new(file);
            let kv: Commands =
                serde_json::from_str(reader.lines().nth(bcv.value_pos).unwrap().ok()?.as_str())
                    .ok()?;
            match kv {
                Commands::Set {
                    t_stamp: _,
                    k_size: _,
                    v_size: _,
                    key: _,
                    value,
                } => Some(value),
                _ => None,
            }
        });
        match res {
            Some(res) => Ok(res),
            None => Ok(None),
        }
        // self.map
        //     .get(key.as_str())
        //     .ok_or(KeyNotFound(key))
        //     .and_then(|bcv| {
        //
        //         Ok(value)
        //     })
    }
    /// remove方法,在键值数据库,删除一个值
    ///
    pub fn remove(&mut self, key: String) -> Result<Option<String>> {
        match self.map.get(key.as_str()) {
            Some(_t) => {
                use chrono::Utc;
                let stamp = Utc::now().timestamp();
                let _ = self.active_file.write(
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
                Ok(Some(key))
            }
            None => Err(DefaultError),
        }
    }
}
