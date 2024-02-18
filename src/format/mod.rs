use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
};

use regex::Regex;

pub struct Formatter;

impl Formatter {
    pub fn format_file(&self, file_path: &str) -> io::Result<()> {
        if !file_path.ends_with(".cs") && !file_path.ends_with(".java") {
            return Ok(());
        }

        let mut content = String::new();
        File::open(file_path)?.read_to_string(&mut content)?;

        println!("before: {:?}", &content);
        self.format_code(&mut content);
        println!("after: {:?}", &content);

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?
            .write_all(content.as_bytes())?;

        println!("Formatted: {:?} {:?}", file_path, content);

        Ok(())
    }

    pub fn format_code(&self, code: &mut String) {
        *code = code
            .replace("  ", "")
            .replace("\n", "")
            .replace("\r", "")
            .replace("{", "{\n")
            .replace("}", "\n}")
    }

    pub fn format(&self, path: &str) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let dir = entry?;
            let file_path = dir.path().to_str().unwrap().to_owned();

            if dir.metadata()?.is_dir() {
                self.format(&file_path)?;
            } else {
                self.format_file(&file_path)?;
            }
        }
        Ok(())
    }
}
