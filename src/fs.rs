use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use crate::config::Config;

pub struct Fs;

impl Fs {
    // TODO: think about refactoring this
    pub fn append_to_file(file_path: String, content: String) {
        let config_dir = Config::read().app.config_dir;
        let file_full_path = format!("{}/{}", config_dir, &file_path);
        Fs::create_file_if_does_not_exist(&file_path);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_full_path)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    pub fn create_file_if_does_not_exist(file_path: &String) {
        let config_dir = Config::read().app.config_dir;
        fs::create_dir_all(&config_dir).expect("Couldn't create directory");
        let file_full_path = format!("{}/{}", config_dir, &file_path);
        let file_exists = Path::new(&file_full_path).exists();
        if !file_exists {
            File::create(file_full_path).unwrap();
        }
    }

    pub fn read_file(file_path: String) -> String {
        let config_dir = Config::read().app.config_dir;
        let file_full_path = format!("{}/{}", config_dir, &file_path);
        let contents = fs::read_to_string(&file_full_path)
            .expect(&format!("Couldn't read file: {}", &file_full_path));
        return contents;
    }
}
