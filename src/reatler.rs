use std::{
    fs::File,
    io::{Read, Write},
    process::exit,
};

pub fn parse_gitignore() -> Vec<String> {
    let mut ignore = Vec::new();
    let mut file = match File::open(".gitignore") {
        Ok(file) => file,
        Err(_) => return ignore,
    };
    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => return ignore,
    }

    for line in buf.lines() {
        ignore.push(line.to_string());
    }
    ignore
}

use crate::dir::{scan_dir, ScanParams};
pub fn parse_args(args: &[String]) -> &str {
    match args.get(1) {
        Some(str) => str,
        None => "./",
    }
}

pub fn run(args: &[String], params: ScanParams) {
    let dir = parse_args(args);

    let file_names = match scan_dir(dir, params) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error while scanning files:  {}", e);
            exit(0);
        }
    };
    add_files(&file_names).expect("Unable to write files");
}

fn add_files(file_names: &Vec<String>) -> Result<(), std::io::Error> {
    let mut output_file = File::create("output.txt").expect("Error occured with output file");

    for file_name in file_names {
        if append_file_to_output(file_name, &mut output_file).is_err() {
            println!("Unable to write {} to output", file_name)
        }
    }
    Ok(())
}

fn append_file_to_output(file_name: &str, output_file: &mut File) -> Result<(), std::io::Error> {
    println!("Inspecting {}", file_name);
    let mut buf = String::new();
    let mut source_file = File::open(file_name).expect("Can't open file for reading");
    source_file
        .read_to_string(&mut buf)
        .expect("Can't read file to string");

    let file_name_line = format!("\n File path: {} \n \n", file_name);
    output_file
        .write_all(file_name_line.as_bytes())
        .expect("Can't write filename to output");
    output_file
        .write_all(buf.as_bytes())
        .expect("Can't write file to output");
    Ok(())
}
