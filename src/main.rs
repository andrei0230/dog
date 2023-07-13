use std::env;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &&str = &args[1].trim();

    match fs::read_to_string(file_path) {
        Ok(content) => println!("{content}"),
        Err(_) => {
            eprintln!("No such file or directory");
            process::exit(1);
        },
    };
}
 