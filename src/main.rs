mod format;
mod code;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Seek, Split, Write},
};

use code::format::{CodeFormatter, FormatRules};



fn main() -> io::Result<()> {
    let format_rules = FormatRules {
        new_line_after: vec!["{".to_string(), ";".to_string()],
        new_line_before: vec!["}".to_string()],
        tab_size: 4,
    };

    let code_formatter = CodeFormatter::new(format_rules);

    code_formatter.format_dir("D:/projects/rust/rusty-format")?;
    Ok(())
}
