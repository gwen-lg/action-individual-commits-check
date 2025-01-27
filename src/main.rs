use std::{
    env,
    fs::File,
    io::Write,
    process::{exit, Command},
};

use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("The `check-cmd` input is not provided")]
    EmptyCheckCmd,
}

fn main() -> anyhow::Result<()> {
    let github_output_path = env::var("GITHUB_OUTPUT").unwrap();
    let mut output_file = File::create(github_output_path).expect("Create output file failed");

    let github_event = env::var("GITHUB_EVENT").unwrap();
    //TODO: if debug
    eprintln!("event={github_event}");

    let args: Vec<String> = env::args().collect();
    let check_cmd = &args[1];
    let error = &args[2];

    if check_cmd.is_empty() {
        eprintln!("Error: a `check-cmd` input should be provided!");
        write!(output_file, "error={error}").unwrap();
        return Err(Error::EmptyCheckCmd.into());
    } else {
        //TODO: create
        eprintln!("Execute `check-cmd`: `{}`", check_cmd);
        let output = Command::new(check_cmd)
            .output()
            .expect("Failed to execute `check-cmd`");
        if output.status.success() {
            write!(output_file, "No error").unwrap();
        } else {
            output_file.write_all(&output.stderr).unwrap();
            exit(1);
        }
    }

    if !error.is_empty() {
        eprintln!("Error: {error}");
        write!(output_file, "error={error}").unwrap();
        exit(1);
    }
    Ok(())
}
