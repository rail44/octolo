use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use clap::{Arg, App, SubCommand, AppSettings};
use serde::{Serialize, Deserialize};
use std::io::{self, Read, Write};
use std::process::Command;
use std::path::Path;
use std::panic;

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

fn get_ghq_root() -> Option<String> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("--path")
        .arg("--null")
        .arg("ghq.root")
        .output()
        .unwrap();
    let mut s = String::from_utf8(output.stdout).unwrap();
    s.pop();
    if s.len() == 0 {
        return None;
    }
    Some(s)
}

fn read_input<R: Read>(mut input: R) -> io::Result<Input> {
    match input.read_u32::<NativeEndian>() {
        Ok(length) => {
            //println!("Found length: {}", length);
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

fn receive() {
    panic::set_hook(Box::new(|p| {
        write_output(io::stdout(), &format!("{}", p)).unwrap();
    }));

    let root = get_ghq_root().unwrap();
    let req = read_input(io::stdin()).unwrap();
    let path = Path::new(&root).join("github.com").join(req.user).join(req.repository).join(req.path);

    let output = Command::new("/usr/local/bin/code")
        .arg("-g")
        .arg(format!("{}:{}", path.to_str().unwrap(), req.line.unwrap_or(0)))
        .output()
        .unwrap();

    let output = Output {
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    };

    write_output(io::stdout(), &output).unwrap();
}

fn manifest() {
}

fn main() {
    let matches = App::new("octoro")
        .setting(AppSettings::AllowExternalSubcommands)
        .version("0.1.0")
        .about("Receive native messaging from browser to open local editor")
        .subcommand(SubCommand::with_name("manifest")
            .unset_setting(AppSettings::AllowExternalSubcommands)
            .about("Generate and place native manifest"))
        .get_matches();
    if let Some(_) = matches.subcommand_matches("manifest") {
        manifest();
    }
    receive();
}
