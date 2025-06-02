use std::{io, process::Command};
use walkdir::WalkDir;

/// Recursively find directories under `root` whose name (case‐insensitive)
/// contains `query`. Honor `ignore` substrings. First tries `fd` if present,
/// otherwise falls back to WalkDir.
pub fn find_directories(root: &str, query: &str, ignore: &[String]) -> io::Result<Vec<String>> {
    let query_lc = query.to_lowercase();

    // 1) Try external `fd` for speed
    if let Ok(output) = Command::new("fd")
        .args(&[
            "--type",
            "d",
            "--hidden",
            "--no-ignore",
            // glob for substring match
            "--glob",
            &format!("*{}*", query),
            root,
        ])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut dirs: Vec<String> = stdout
                .lines()
                .map(str::to_string)
                .filter(|p| !p.is_empty())
                .collect();
            // filter by our ignore list
            dirs.retain(|p| !ignore.iter().any(|pat| p.contains(pat)));
            if !dirs.is_empty() {
                return Ok(dirs);
            }
        }
    }

    // 2) Fallback to WalkDir
    let mut dirs = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let ps = path.to_string_lossy();
        if ignore.iter().any(|pat| ps.contains(pat)) {
            continue;
        }
        if let Some(name) = path.file_name().map(|s| s.to_string_lossy().to_lowercase()) {
            if name.contains(&query_lc) {
                dirs.push(ps.to_string());
            }
        }
    }
    Ok(dirs)
}

/// Prompt the user to pick one of the `dirs` by number.
pub fn choose_directory(dirs: &[String]) -> Option<String> {
    use std::io::{self, Write};
    println!("\nFound {} matching directories:", dirs.len());
    for (i, d) in dirs.iter().enumerate() {
        println!("  {}) {}", i + 1, d);
    }
    print!("\nEnter choice number: ");
    io::stdout().flush().ok()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    let n = buf.trim().parse::<usize>().ok()?;
    if n == 0 || n > dirs.len() {
        None
    } else {
        Some(dirs[n - 1].clone())
    }
}
