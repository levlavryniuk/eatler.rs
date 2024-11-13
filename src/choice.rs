use std::io::Write;

pub fn get_types() -> Vec<String> {
    print!("\nWhich file types to include? (example: rs toml) ");
    std::io::stdout().flush().expect("Could not flush stdin");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read from stdin");

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
    std::io::stdout().flush().expect("Could not flush stdin");
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read from stdin");

    let buf: Vec<String> = buf.split(' ').map(|val| val.trim().to_string()).collect();
    buf
}
