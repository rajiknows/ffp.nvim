use std::io::{self, Write};

fn main() {
    for entry in walkdir::WalkDir::new(".") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
    io::stdout().flush().unwrap();
}
