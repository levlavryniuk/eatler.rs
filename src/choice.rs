use std::{io::Write, process::exit};

#[derive(Default)]
pub enum ScanType {
    Manual,
    #[default]
    Auto,
}
pub fn get_types() -> Vec<String> {
    print!("\nWhich file formats to include? (example: rs toml json) ");
    std::io::stdout().flush().expect("Could not flush stdin");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read from stdin");

    if buf.trim().is_empty() {
        println!("No file formats specified, exiting...");
        exit(1)
    }
    let buf: Vec<String> = buf
        .split(' ')
        .map(|val| val.trim().to_string())
        .map(|val| {
            if val.starts_with('.') {
                val
            } else {
                format!(".{}", val)
            }
        })
        .collect();

    buf
}

pub fn get_ignore() -> Vec<String> {
    print!("\nWhich files/directories to ignore? (example: target dist .d.ts) ");
    println!("Note: .gitignore parsing enabled");
    std::io::stdout().flush().expect("Could not flush stdin");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read from stdin");

    if buf.trim().is_empty() {
        println!("No files or directories specified");
        return vec![".git".into()];
    }
    let mut buf: Vec<String> = buf.split(' ').map(|val| val.trim().to_string()).collect();
    buf.push(".git".into());
    buf
}

pub fn get_scan_type(args: &[String]) -> ScanType {
    if args.iter().any(|v| v == "--manual") {
        ScanType::Manual
    } else {
        ScanType::Auto
    }
}
