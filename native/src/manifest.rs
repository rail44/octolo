use std::env::current_exe;
use std::path::PathBuf;
use std::fs::{create_dir_all, File};
use serde::Serialize;
use dirs::home_dir;

#[derive(Serialize)]
struct FirefoxManifest {
    name: String,
    description: String,
    path: String,
    #[serde(rename = "type")]
    _type: String,
    allowed_extensions: Vec<String>,
}

impl FirefoxManifest {
    pub fn new() -> Result<Self, failure::Error> {
        let path = current_exe()?.to_str().unwrap().to_string();
        Ok(FirefoxManifest {
            name: "jp.rail44.octolo".to_string(),
            description: "Open files with local editor from GitHub web".to_string(),
            path: path,
            _type: "stdio".to_string(),
            allowed_extensions: vec!["{3de89a2b-180a-427e-85dd-29c983e93ba3}".to_string()]
        })
    }
}

static FILE_NAME: &str = "jp.rail44.octolo.json";

fn get_firefox_manifest_dir(home: PathBuf) -> PathBuf {
    home.join("Library/Application Support/Mozilla/NativeMessagingHosts")
}

pub fn manifest() -> Result<(), failure::Error> {
    let home = home_dir().ok_or(crate::Error::CouldNotDetermineHomeDir)?;
    let path = get_firefox_manifest_dir(home);
    create_dir_all(&path)?;
    let file = File::create(path.join(FILE_NAME))?;
    serde_json::to_writer_pretty(file, &FirefoxManifest::new()?)?;
    Ok(())
}

