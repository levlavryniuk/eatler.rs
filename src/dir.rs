use std::{fs::read_dir, path::Path};
#[derive(Default)]
pub struct ScanParams {
    pub ignore: Vec<String>,
    pub include: Vec<String>,
}
/// Scans a directory for files that match the include and ignore parameters
pub fn scan_dir(
    dirname: &str,
    params: ScanParams,
    recursive: bool,
) -> std::io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    dir_helper(dirname, &mut files, &params, recursive)?;
    Ok(files)
}

fn dir_helper(
    dirname: &str,
    files: &mut Vec<String>,
    params: &ScanParams,
    recursive: bool,
) -> std::io::Result<()> {
    let ScanParams { include, ignore } = params;
    for entry in read_dir(dirname)? {
        let entry = entry?;
        let path = entry.path();
        let is_dir = path.is_dir();
        let path_str = path.to_string_lossy().to_string();

        if is_ignored(&path_str, ignore) {
            continue;
        }

        if is_dir && recursive {
            dir_helper(&path_str, files, params, true)?;
        } else if is_included(&path_str, include) {
            files.push(path_str);
        }
    }

    Ok(())
}

/// Normalize a gitignore pattern to handle different path formats
fn normalize_pattern(pattern: &str) -> String {
    let pattern = pattern.trim();
    let pattern = pattern.strip_prefix("./").unwrap_or(pattern);
    let pattern = pattern.strip_prefix('/').unwrap_or(pattern);
    pattern.to_string()
}

/// Checks if a file should be ignored based on gitignore patterns
fn is_ignored(path: &str, ignored: &[String]) -> bool {
    let path = Path::new(path);
    let path_str = path.to_string_lossy();

    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();

    for pattern in ignored {
        let normalized_pattern = normalize_pattern(pattern);

        if path_str.contains(&normalized_pattern) {
            return true;
        }

        if file_name.contains(&normalized_pattern) {
            return true;
        }
    }

    false
}
/// Updated is_included to match against the full path
fn is_included(path: &str, included: &[String]) -> bool {
    let path = Path::new(path);
    if let Some(file_name) = path.file_name() {
        let file_name = file_name.to_string_lossy();
        included
            .iter()
            .any(|included| file_name.ends_with(included))
    } else {
        false
    }
}
