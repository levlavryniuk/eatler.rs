mod choice;
mod dir;
mod project_type;
mod reatler;
mod smart;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    reatler::run(&args);
}
