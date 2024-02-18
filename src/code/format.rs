use std::{fs::{self, File, OpenOptions}, io::{self, Read, Write}, ops::Index};

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FormatRules {
    pub new_line_after: Vec<String>,
    pub new_line_before: Vec<String>,
    pub tab_size: u8,
}

pub struct CodeFormatter {
    format_rules: FormatRules,
}

impl CodeFormatter {
    pub fn new(format_rules: FormatRules) -> Self {
        Self {
            format_rules
        }
    }

    pub fn format_dir(&self, dir: &str) -> io::Result<()>{
         for entry in fs::read_dir(dir)? {
            let dir = entry?;
            let file_path = dir.path().to_str().unwrap().to_owned();

            if dir.metadata()?.is_dir() {
                self.format_dir(&file_path)?;
            } else {
                self.format_file(&file_path)?;
            }
        }
        Ok(())
    }   

    pub fn format_file(&self, file_path: &str) -> io::Result<()> {
         if !file_path.ends_with(".cs") && !file_path.ends_with(".java") {
            return Ok(());
        }

        let mut content = String::new();
        File::open(file_path)?.read_to_string(&mut content)?;

        println!("before: {:?}", &content);
        let formatted_code = self.format_code(&content);
        println!("after: {:?}", &formatted_code);

        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?
            .write_all(formatted_code.as_bytes())?;

        println!("Formatted: {:?} {:?}", file_path, formatted_code);

        Ok(())
    }

    pub fn format_code(&self, code: &str) -> String {
        let mut formatted_code = code.to_owned()
            .replace("  ", "")
            .replace("{\n", "")
            .replace("\n{", "")
            .replace("\r", "");

        for new_line_after in &self.format_rules.new_line_after {
            let mut format_to = new_line_after.clone();
            format_to.push_str("\n");
            formatted_code = formatted_code.replace(new_line_after, &format_to);
        }

        for new_line_before in &self.format_rules.new_line_before {
            let mut format_to = "\n".to_string();
            format_to.push_str(&new_line_before);
            formatted_code = formatted_code.replace(new_line_before, &format_to);
        }

        formatted_code
    }

    fn insert_tab(code: &mut String) {

    }
}
