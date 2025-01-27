use std::{
    env,
    fs::File,
    io::Write,
    process::{exit, Command},
};

fn main() {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();
    let mut output_file = File::create(github_output_path).expect("Create output file failed");

    let args: Vec<String> = env::args().collect();
    let error = &args[1];

    if !error.is_empty() {
        eprintln!("Error: {error}");
        write!(output_file, "error={error}").unwrap();
        exit(1);
    }
}
