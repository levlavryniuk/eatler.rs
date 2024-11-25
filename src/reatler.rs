use crate::{
    choice::{self, get_scan_type, ScanType},
    dir::{scan_dir, ScanParams},
    project_type::ProjectType,
};
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
        if !line.is_empty() {
            ignore.push(line.to_string());
        }
    }
    ignore
}

pub fn parse_args(args: &[String]) -> &str {
    match args.get(1) {
        Some(str) => str,
        None => "./",
    }
}

pub fn run(args: &[String]) {
    let dir = parse_args(args);
    let is_manual = matches!(get_scan_type(args), ScanType::Manual);

    let params = if is_manual {
        get_scan_params_manual()
    } else {
        match get_scan_params_auto(dir) {
            Some(p) => p,
            None => {
                println!("Type autodiscovery failed, fallback to manual selection");
                get_scan_params_manual()
            }
        }
    };

    let file_names = match scan_dir(dir, params, true) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error while scanning files:  {}", e);
            exit(0);
        }
    };
    for file in &file_names {
        println!("+{}", file)
    }
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
fn get_scan_params_manual() -> ScanParams {
    let include = choice::get_types();
    let mut ignore = choice::get_ignore();
    let gitignore = parse_gitignore();
    ignore.extend(gitignore);
    ScanParams { include, ignore }
}
fn get_scan_params_auto(dir: &str) -> Option<ScanParams> {
    let mut params = ScanParams::default();
    let gitignore = parse_gitignore();

    let tmp_params = ScanParams {
        include: vec!["".into()],
        ignore: gitignore.clone(),
    };

    let files = scan_dir(dir, tmp_params, false).unwrap_or_default();
    let types = get_project_types(&files)?;

    params.ignore.extend(gitignore);
    params.include.extend(
        types
            .iter()
            .flat_map(|project_type| project_type.get_files())
            .map(|p_t| p_t.to_string()),
    );
    Some(params)
}

fn get_project_types(files: &Vec<String>) -> Option<Vec<ProjectType>> {
    let mut types: Vec<ProjectType> = Vec::new();
    for file in files {
        match file.split("/").last() {
            Some(striped) => types.extend(ProjectType::from(striped)),
            None => println!("Bad shit happned with {}", file),
        }
    }

    types.dedup();

    if types.is_empty() {
        return None;
    }
    println!(
        "A {} project found!",
        types
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    Some(types)
}
