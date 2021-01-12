use bincode;
use chrono::Duration;
use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Error, path::PathBuf};

const STORAGE_PATH: &str = "/var/cache/dpm/";

pub struct DataStore<T> {
    pub data: T,
    pub path: PathBuf,
    pub path_str: String,
}

impl<T: Serialize + DeserializeOwned + Clone> DataStore<T> {
    pub fn new(name: &str, default: &T) -> Result<Self, Error> {
        let path_str = format!("{}{}", STORAGE_PATH, name);
        let path = PathBuf::from(&*path_str);

        if path.is_dir() {
            panic!("{} already exists as a directory", path.to_str().unwrap());
        }

        let mut data;

        if path.exists() {
            let file = File::open(&path)?;
            data = bincode::deserialize_from(file).unwrap();
        } else {
            data = default.clone();
        }

        Ok(DataStore {
            path,
            data,
            path_str,
        })
    }
}