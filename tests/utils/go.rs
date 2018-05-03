use std;
use std::io::Write;
use std::process::Output;
use tempfile::Builder;

#[derive(Debug, Fail)]
pub enum GolangError {
    #[fail(display = "unable to create temporary file for golang program code: {}", _0)]
    CodeFileNotCreated(#[cause] std::io::Error),
    #[fail(display = "unable to write golang code: {}", _0)]
    CodeFileNotWritten(#[cause] std::io::Error),
    #[fail(display = "{}", _0)]
    LaunchFailed(#[cause] std::io::Error),
    #[fail(
        display = "go binary returned an error: status code {}.\nstderr: {}\nstdout: {}\n",
        status,
        stderr,
        stdout
    )]
    Runtime {
        status: i32,
        stderr: String,
        stdout: String,
    },
}

pub fn run(code: &str) -> Result<Output, GolangError> {
    run_with_input(code, None as Option<Vec<u8>>)
}

pub fn run_with_input<T: Into<Vec<u8>>>(
    code: &str,
    input: Option<T>,
) -> Result<Output, GolangError> {
    // Write the go code to a temp file.
    let mut tmp = Builder::new()
        .prefix("rust_gob_test")
        .suffix(".go")
        .tempfile()
        .map_err(GolangError::CodeFileNotCreated)?;
    tmp.write_all(code.as_bytes())
        .map_err(GolangError::CodeFileNotWritten)?;

    // We are going to run the go code and read `stdout` and `stderr`.
    let mut command = cmd!("go", "run", tmp.path())
        .stdout_capture()
        .stderr_capture();

    // Write data to `stdin` if told to do so.
    if let Some(i) = input {
        command = command.input(i);
    };

    // Actually run the go code.
    match command.unchecked().run() {
        Err(e) => Err(GolangError::LaunchFailed(e)),
        Ok(r) => {
            if r.status.success() {
                return Ok(r);
            }
            Err(GolangError::Runtime {
                status: r.status.code().unwrap_or(-99),
                stderr: String::from_utf8_lossy(&r.stderr).to_string(),
                stdout: String::from_utf8_lossy(&r.stdout).to_string(),
            })
        }
    }
}
