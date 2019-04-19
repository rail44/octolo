#![feature(custom_attribute)]

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use clap::{value_t_or_exit, App, AppSettings, Arg, SubCommand};
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::panic;
use std::path::Path;
use std::process::Command;

mod browser;
use browser::{manifest, Browser};

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

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Could not get ghq.root")]
    CouldNotGetGhqRoot,
    #[fail(display = "Could not determine home dir")]
    CouldNotDetermineHomeDir,
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
        return Err(Error::CouldNotGetGhqRoot.into());
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

fn receive() -> Result<(), failure::Error> {
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

fn main() {
    let matches = App::new("octoro")
        .setting(AppSettings::AllowExternalSubcommands)
        .version("0.1.0")
        .about("Receive native messaging from browser to open local editor")
        .subcommand(
            SubCommand::with_name("manifest")
                .unset_setting(AppSettings::AllowExternalSubcommands)
                .about("Generate and place native manifest")
                .arg(
                    Arg::with_name("browser")
                        .possible_values(&Browser::variants())
                        .short("b")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("write")
                        .short("w")
                        .help("Write manifest insted of show"),
                ),
        )
        .get_matches();
    if let Some(c) = matches.subcommand_matches("manifest") {
        let browser = value_t_or_exit!(c.value_of("browser"), Browser);
        let write = c.is_present("write");
        manifest(browser, write).unwrap();
        return;
    }
    receive().unwrap();
}
