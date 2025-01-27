use std::{
    env,
    fs::File,
    io::{self, Write},
    process::Command,
};

use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("The `check-cmd` input is not provided")]
    EmptyCheckCmd,

    #[error("Launch of check-cmd : {cmd} failed.")]
    CommandLaunchFailed {
        #[source]
        source: io::Error,
        cmd: String,
    },

    #[error("Running of check-cmd : {cmd} failed.")]
    CommandExecutionFailed { cmd: String },

    #[error("The test error was triggered")]
    TestGhAction,
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
        let output =
            Command::new(check_cmd)
                .output()
                .map_err(|err| Error::CommandLaunchFailed {
                    source: err,
                    cmd: check_cmd.into(),
                })?;
        if output.status.success() {
            write!(output_file, "No error").unwrap();
        } else {
            output_file.write_all(&output.stderr).unwrap();
            return Err(Error::CommandExecutionFailed {
                cmd: check_cmd.into(),
            }
            .into());
        }
    }

    if !error.is_empty() {
        eprintln!("Error: {error}");
        write!(output_file, "error={error}").unwrap();
        return Err(Error::TestGhAction.into());
    }
    Ok(())
}
