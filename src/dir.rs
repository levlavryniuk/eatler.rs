use std::fs::read_dir;
pub struct ScanParams {
    pub ignore: Vec<String>,
    pub include: Vec<String>,
}
/// Checks if a file should be ignored in the scan
fn check_ignore(file_name: &str, ignored: &[String]) -> bool {
    ignored.iter().any(|ignored| *ignored == file_name)
}
/// Checks if a file should be included in the scan
fn check_include(file_name: &str, included: &[String]) -> bool {
    included
        .iter()
        .any(|included| file_name.ends_with(included))
}
/// Scans a directory for files that match the include and ignore parameters
pub fn scan_dir(dirname: &str, params: ScanParams) -> std::io::Result<Vec<String>> {
    let mut files: Vec<String> = Vec::new();
    dir_helper(dirname, &mut files, &params)?;
    Ok(files)
}
/// Helper function for scan_dir
fn dir_helper(dirname: &str, files: &mut Vec<String>, params: &ScanParams) -> std::io::Result<()> {
    let ScanParams { include, ignore } = params;
    for entry in read_dir(dirname)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().into_string().expect("Unlick");

        let is_dir = path.is_dir();
        let path = path.to_str().expect("Path is not valid unicode");

        if check_ignore(&file_name, ignore) {
            continue;
        }

        if is_dir {
            dir_helper(path, files, params)?;
        } else if check_include(&file_name, include) {
            files.push(path.to_string());
        }
    }

    Ok(())
}
