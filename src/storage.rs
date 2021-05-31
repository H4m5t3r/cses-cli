use anyhow::Result;
use miniserde::{json, Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILENAME: &str = "filestorage.json";

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FileStorage {
    token: Option<String>,
    course: Option<String>,
    task: Option<u64>,
    language: Option<String>,
    option: Option<String>,
    file: Option<String>,
}

impl FileStorage {
    pub fn new() -> Self {
        if !Path::new(FILENAME).exists() {
            return Default::default();
        };
        let data = fs::read_to_string(FILENAME).unwrap();
        let res = json::from_str(&data);
        match res {
            Ok(fs) => fs,
            Err(_) => Default::default(),
        }
    }
}

pub trait Storage {
    fn get_token(&self) -> Option<&str>;
    fn get_course(&self) -> Option<&str>;
    fn get_task(&self) -> Option<u64>;
    fn get_language(&self) -> Option<&str>;
    fn get_option(&self) -> Option<&str>;
    fn get_file(&self) -> Option<&str>;
    fn set_token(&mut self, val: String);
    fn set_course(&mut self, val: String);
    fn set_task(&mut self, val: u64);
    fn set_language(&mut self, val: String);
    fn set_option(&mut self, val: String);
    fn set_file(&mut self, val: String);
    fn save(&mut self) -> Result<()>;
    fn delete(&mut self) -> Result<()>;
}

impl Storage for FileStorage {
    fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    fn get_course(&self) -> Option<&str> {
        self.course.as_deref()
    }
    fn get_task(&self) -> Option<u64> {
        self.task
    }
    fn get_language(&self) -> Option<&str> {
        self.language.as_deref()
    }
    fn get_option(&self) -> Option<&str> {
        self.option.as_deref()
    }
    fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
    }
    fn set_token(&mut self, val: String) {
        self.token = Some(val);
    }
    fn set_course(&mut self, val: String) {
        self.course = Some(val);
    }
    fn set_task(&mut self, val: u64) {
        self.task = Some(val);
    }
    fn set_language(&mut self, val: String) {
        self.language = Some(val);
    }
    fn set_option(&mut self, val: String) {
        self.option = Some(val);
    }
    fn set_file(&mut self, val: String) {
        self.file = Some(val);
    }
    fn save(&mut self) -> Result<()> {
        Ok(fs::write(FILENAME, json::to_string(self))?)
    }
    fn delete(&mut self) -> Result<()> {
        Ok(fs::remove_file(FILENAME)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setters_and_getters_work() {
        let mut storage: FileStorage = Default::default();
        storage.set_token(String::from("token"));
        storage.set_course(String::from("course"));
        storage.set_task(42);
        storage.set_language(String::from("language"));
        storage.set_option(String::from("option"));
        storage.set_file(String::from("file"));
        assert_eq!(String::from("token"), storage.get_token().unwrap());
        assert_eq!(String::from("course"), storage.get_course().unwrap());
        assert_eq!(42, storage.get_task().unwrap());
        assert_eq!(String::from("language"), storage.get_language().unwrap());
        assert_eq!(String::from("option"), storage.get_option().unwrap());
        assert_eq!(String::from("file"), storage.get_file().unwrap());
    }
}
