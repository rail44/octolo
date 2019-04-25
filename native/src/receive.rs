use crate::config::Config;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::panic;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Input {
    Open {
        user: String,
        repository: String,
        revision: String,
        path: String,
        line: Option<i32>,
    },
    GetConfig,
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

pub fn receive(config: Config) -> Result<(), failure::Error> {
    panic::set_hook(Box::new(|p| {
        write_output(
            io::stdout(),
            &ErrorOutput {
                error: format!("{}", p).to_string(),
            },
        )
        .unwrap();
    }));
    let input = read_input(io::stdin())?;
    let command_output = config.get_command(&input)?.output()?;
    let output = Output {
        stdout: String::from_utf8(command_output.stdout)?,
        stderr: String::from_utf8(command_output.stderr)?,
    };
    write_output(io::stdout(), &output)?;
    Ok(())
}
