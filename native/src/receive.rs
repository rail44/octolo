use crate::config::Config;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::panic;

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
enum Message {
    Open(OpenMessage),
    GetConfig,
}

#[derive(Deserialize, Serialize)]
pub struct OpenMessage {
    pub user: String,
    pub repository: String,
    pub revision: String,
    pub path: String,
    #[serde(default = "get_default_line")]
    pub line: i32,
    pub editor: String,
}

fn get_default_line() -> i32 {
    0
}

#[derive(Serialize)]
struct ErrorOutput {
    error: String,
}

#[derive(Serialize)]
struct ConfigOutput {
    editor_list: Vec<EditorOutput>,
}

#[derive(Serialize)]
struct EditorOutput {
    shortcut: Option<String>,
    kind: String,
    label: String,
}

impl From<Config> for ConfigOutput {
    fn from(c: Config) -> Self {
        ConfigOutput {
            editor_list: c
                .editors
                .iter()
                .map(|e| EditorOutput {
                    shortcut: e.shortcut.clone(),
                    kind: e.get_kind(),
                    label: e.get_label(),
                })
                .collect(),
        }
    }
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
            let output = config.open(&message)?;
            Ok(write_output(io::stdout(), &output)?)
        }
        Message::GetConfig => {
            let output = ConfigOutput::from(config);
            Ok(write_output(io::stdout(), &output)?)
        }
    }
}
