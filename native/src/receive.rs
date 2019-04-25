use crate::config::Config;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::panic;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
enum Message{
    Open(OpenMessage),
    GetConfig
}

#[derive(Deserialize, Serialize)]
pub struct OpenMessage {
    pub user: String,
    pub repository: String,
    pub revision: String,
    pub path: String,
    pub line: Option<i32>,
    pub editor: String,
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

fn read_input<R: Read>(mut message: R) -> io::Result<Message> {
    match message.read_u32::<NativeEndian>() {
        Ok(length) => {
            let mut buffer = vec![0; length as usize];
            message.read_exact(&mut buffer)?;
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
    match read_input(io::stdin())? {
        Message::Open(message) => {
            let command_output = config.get_command(&message)?.output()?;
            let output = Output {
                stdout: String::from_utf8(command_output.stdout)?,
                stderr: String::from_utf8(command_output.stderr)?,
            };
            Ok(write_output(io::stdout(), &output)?)
        },
        Message::GetConfig => {
            Ok(write_output(io::stdout(), &config)?)
        }
    }
}
