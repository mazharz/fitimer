use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

pub struct Fs;

impl Fs {
    pub fn append_to_file(path: String, content: String) {
        Fs::create_file_if_does_not_exist(&path);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
        if let Err(e) = writeln!(file, "{}", content) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    fn create_file_if_does_not_exist(path: &String) {
        let path = PathBuf::from(path);
        let dir = path.parent().unwrap().to_str().unwrap();
        fs::create_dir_all(&dir).expect("Couldn't create directory");
        let file_exists = Path::new(&path).exists();
        if !file_exists {
            File::create(path).unwrap();
        }
    }

    pub fn read_file(path: String) -> Result<Vec<String>, String> {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                let err = format!("Couldn't read file: {}", &path);
                return Err(err);
            }
        };
        let reader = BufReader::new(file);
        let lines = reader
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .expect(&format!("Couldn't parse file lines: {}", &path));
        return Ok(lines);
    }

    pub fn read_file_str(path: String) -> String {
        let mut file = File::open(&path).expect(&format!("Couldn't read file: {}", &path));
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Couldn't read file contents.");
        return file_contents;
    }
}
