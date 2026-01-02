use arboard::{Clipboard, SetExtLinux};

use crate::{
    choice::{self, get_scan_type, ScanType},
    dir::{scan_dir, ScanParams},
    project_type::ProjectType,
};
use std::{
    fs::{read_to_string, File},
    io::{Read, Write},
    process::exit,
    time::Duration,
};

use crate::smart;
pub fn parse_gitignore() -> Vec<String> {
    let mut ignore = Vec::new();
    if let Ok(mut f) = File::open(".gitignore") {
        let mut buf = String::new();
        if f.read_to_string(&mut buf).is_ok() {
            buf.lines()
                .filter(|l| !l.trim().is_empty())
                .for_each(|l| ignore.push(l.to_string()));
        }
    }
    ignore
}

/// Returns (target_dir, optional_smart_query)
fn parse_args(args: &[String]) -> (String, Option<String>) {
    let mut dir = "./".to_string();
    let mut smart_q = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--smart" => {
                if let Some(q) = args.get(i + 1) {
                    smart_q = Some(q.clone());
                    i += 2;
                } else {
                    eprintln!("--smart requires a query");
                    exit(1);
                }
            }
            flag if flag == "--manual" || flag == "-m" => {
                // leave it to get_scan_type()
                i += 1;
            }
            other if other.starts_with('-') => {
                // ignore unknown flags
                i += 1;
            }
            path => {
                dir = path.to_string();
                i += 1;
            }
        }
    }
    (dir, smart_q)
}

pub fn run(args: &[String]) {
    // 1) parse args
    let (mut dir, smart_q) = parse_args(args);

    // 2) maybe do smart lookup
    if let Some(query) = smart_q {
        let gitignore = parse_gitignore();
        let matches = match smart::find_directories(&dir, &query, &gitignore) {
            Ok(v) if !v.is_empty() => v,
            _ => {
                eprintln!("No directories matching “{}” found under {}", query, dir);
                exit(1)
            }
        };
        let choice = smart::choose_directory(&matches).unwrap_or_else(|| {
            eprintln!("Invalid selection, aborting.");
            exit(1)
        });
        println!("\n→ Assembling files under: {}\n", choice);
        dir = choice;
    }

    // 3) manual vs auto
    let is_manual = matches!(get_scan_type(args), ScanType::Manual);
    let params = if is_manual {
        get_scan_params_manual()
    } else {
        get_scan_params_auto(&dir).unwrap_or_else(|| {
            println!("Auto-detection failed, falling back to manual.");
            get_scan_params_manual()
        })
    };

    // 4) scan
    let files = match scan_dir(&dir, params, true) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error scanning files: {}", e);
            exit(1)
        }
    };
    for f in &files {
        println!("+{}", f);
    }

    // 5) output
    if let Err(e) = add_files(&files) {
        eprintln!("Error writing output: {}", e);
        exit(1)
    }
}

fn add_files(files: &[String]) -> Result<(), std::io::Error> {
    let mut out = File::create("output.txt")?;
    for f in files {
        append_file_to_output(f, &mut out)?;
    }
    if let Ok(mut clip) = Clipboard::new() {
        println!("Copied to clipboard\nPress ctrl+c when finished pasting");
        match clip.set().wait().file_list(&["output.txt"]) {
            Ok(_) => println!("Copied file path"),
            Err(e) => println!("Error when tried to copy file to clipboard {}", e),
        };
    } else {
        println!("Can't reach clipboard");
    };
    Ok(())
}

fn append_file_to_output(file_name: &str, output_file: &mut File) -> Result<(), std::io::Error> {
    let mut buf = String::new();
    let mut src = File::open(file_name)?;
    src.read_to_string(&mut buf)?;
    writeln!(output_file, "\n File path: {}\n", file_name)?;
    output_file.write_all(buf.as_bytes())?;
    Ok(())
}

fn get_scan_params_manual() -> ScanParams {
    let mut ignore = choice::get_ignore();
    ignore.extend(parse_gitignore());
    ScanParams {
        include: choice::get_types(),
        ignore,
    }
}

fn get_scan_params_auto(dir: &str) -> Option<ScanParams> {
    let gitignore = parse_gitignore();
    let dry = ScanParams {
        include: vec!["".into()],
        ignore: gitignore.clone(),
    };
    let files = scan_dir(dir, dry, false).unwrap_or_default();
    let mut types = get_project_types(&files)?;
    types.dedup();
    let mut params = ScanParams::default();
    params.ignore.extend(gitignore);
    params
        .include
        .extend(types.iter().flat_map(|t| t.get_files()).map(String::from));
    Some(params)
}

fn get_project_types(files: &[String]) -> Option<Vec<ProjectType>> {
    let mut types = Vec::new();
    for file in files {
        if let Some(name) = file.rsplit('/').next() {
            types.extend(ProjectType::from(name));
        }
    }
    types.dedup();
    if types.is_empty() {
        return None;
    }
    println!(
        "Detected project type(s): {}",
        types
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    Some(types)
}
