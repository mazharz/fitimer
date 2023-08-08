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
            Err(_) => return Err(format!("Couldn't read file: {}", &path)),
        };
        let reader = BufReader::new(file);
        match reader.lines().collect::<Result<Vec<String>, _>>() {
            Ok(lines) => return Ok(lines),
            Err(_) => return Err(format!("Couldn't parse file lines: {}", &path)),
        }
    }

    pub fn read_file_str(path: String) -> Result<String, String> {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Couldn't read file: {}", &path)),
        };
        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Ok(_) => return Ok(file_contents),
            Err(_) => return Err(String::from("Couldn't read file contents.")),
        }
    }
}
