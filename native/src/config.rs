use crate::browser::Browser;
use crate::receive::Input;
use dirs::config_dir;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

static FILE_PATH: &str = "octolo/octolo.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub browser_list: Vec<Browser>,
    #[serde(default = "get_default_root")]
    pub root: String,
    #[serde(default = "get_default_path")]
    pub path: String,
    pub editor: Editor,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum Editor {
    #[serde(rename = "visual-studio-code")]
    VisualStudioCode {
        bin: String,
        #[serde(default = "visual_studio_code_args")]
        args: Vec<String>,
    },
    #[serde(rename = "neovim-remote")]
    NeovimRemote {
        bin: String,
        #[serde(default = "neovim_remote_args")]
        args: Vec<String>,
    },
    Other {
        cmd: Vec<String>,
    },
}

impl Config {
    pub fn get_command(&self, input: &Input) -> Result<Command, failure::Error> {
        match &self.editor {
            Editor::VisualStudioCode { bin, args } => {
                self.get_command_from_bin_and_args(bin, args, input)
            }
            Editor::NeovimRemote { bin, args } => {
                self.get_command_from_bin_and_args(bin, args, input)
            }
            Editor::Other { cmd } => {
                let (bin, args) = cmd.split_at(0);
                let bin = bin.first().unwrap();
                self.get_command_from_bin_and_args(bin, args, input)
            }
        }
    }

    fn get_command_from_bin_and_args(
        &self,
        bin: &str,
        args: &[String],
        input: &Input,
    ) -> Result<Command, failure::Error> {
        let h = Handlebars::new();
        let mut c = Command::new(bin);
        c.current_dir(Path::new(&self.root).join(&h.render_template(&self.path, &input)?));
        c.args(
            args.iter()
                .map(|a| h.render_template(a, &input))
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(c)
    }
}

fn visual_studio_code_args() -> Vec<String> {
    vec!["-g".to_string(), "{{path}}:{{line}}".to_string()]
}

fn neovim_remote_args() -> Vec<String> {
    vec![
        "--nostart".to_string(),
        "-p".to_string(),
        "{{path}}".to_string(),
    ]
}

fn get_file_path() -> Result<PathBuf, failure::Error> {
    Ok(config_dir()
        .ok_or(crate::Error::CouldNotDetermineConfigDir)?
        .join(FILE_PATH))
}

fn get_default_root() -> String {
    get_ghq_root().unwrap()
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

fn get_default_path() -> String {
    "github.com/{{user}}/{{repository}}".to_string()
}

pub fn read_config() -> Result<Config, failure::Error> {
    let path = get_file_path()?;
    let mut s = String::new();
    let mut r = BufReader::new(File::open(path)?);
    r.read_to_string(&mut s)?;
    Ok(toml::from_str(&s)?)
}
