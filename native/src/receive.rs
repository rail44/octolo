use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::panic;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Input {
    user: String,
    repository: String,
    revision: String,
    path: String,
    line: Option<i32>,
}

#[derive(Serialize)]
struct Output {
    stdout: String,
    stderr: String,
}

#[derive(Serialize)]
struct ErrorOutput {
    error: String,
}

fn get_ghq_root() -> Result<String, failure::Error> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("--path")
        .arg("--null")
        .arg("ghq.root")
        .output()?;
    let mut s = String::from_utf8(output.stdout)?;
    s.pop();
    if s.is_empty() {
        return Err(crate::Error::CouldNotGetGhqRoot.into());
    }
    Ok(s)
}

fn read_input<R: Read>(mut input: R) -> io::Result<Input> {
    match input.read_u32::<NativeEndian>() {
        Ok(length) => {
            let mut buffer = vec![0; length as usize];
            input.read_exact(&mut buffer)?;
            let value = serde_json::from_slice(&buffer)?;
            Ok(value)
        }
        _ => panic!(),
    }
}

fn write_output<W: Write, S: Serialize>(mut output: W, value: &S) -> io::Result<()> {
    let msg = serde_json::to_string(value)?;
    let len = msg.len();
    output.write_u32::<NativeEndian>(len as u32)?;
    output.write_all(msg.as_bytes())?;
    output.flush()?;
    Ok(())
}

pub fn receive() -> Result<(), failure::Error> {
    panic::set_hook(Box::new(|p| {
        write_output(
            io::stdout(),
            &ErrorOutput {
                error: format!("{}", p).to_string(),
            },
        )
        .unwrap();
    }));

    let root = get_ghq_root()?;
    let req = read_input(io::stdin())?;
    let path = Path::new(&root)
        .join("github.com")
        .join(req.user)
        .join(req.repository)
        .join(req.path);

    let output = Command::new("/usr/local/bin/code")
        .arg("-g")
        .arg(format!(
            "{}:{}",
            path.to_str().unwrap(),
            req.line.unwrap_or(0)
        ))
        .output()?;

    let output = Output {
        stdout: String::from_utf8(output.stdout)?,
        stderr: String::from_utf8(output.stderr)?,
    };

    write_output(io::stdout(), &output)?;
    Ok(())
}
