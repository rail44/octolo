use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{stdout, BufWriter};

#[derive(Deserialize, Serialize)]
pub enum Browser {
    FireFox,
    Chrome,
    Chromium,
}

impl fmt::Display for Browser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Browser::FireFox => write!(f, "FireFox"),
            &Browser::Chrome => write!(f, "Chrome"),
            &Browser::Chromium => write!(f, "Chromium"),
        }
    }
}

impl Browser {
    #[cfg(target_os = "macos")]
    pub fn get_manifest_dir(&self) -> String {
        match self {
            Browser::FireFox => {
                "Library/Application Support/Mozilla/NativeMessagingHosts".to_string()
            }
            Browser::Chrome => {
                "Library/Application Support/Google/Chrome/NativeMessagingHosts".to_string()
            }
            Browser::Chromium => {
                "Library/Application Support/Chromium/NativeMessagingHosts".to_string()
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn get_manifest_dir(&self) -> String {
        match self {
            Browser::FireFox => ".mozilla/native-messaging-hosts".to_string(),
            Browser::Chrome => ".config/google-chrome/NativeMessagingHosts".to_string(),
            Browser::Chromium => ".config/chromium/NativeMessagingHosts".to_string(),
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum Manifest {
    FireFox {
        name: String,
        description: String,
        path: String,
        #[serde(rename = "type")]
        _type: String,
        allowed_extensions: Vec<String>,
    },
    Chrome {
        name: String,
        description: String,
        path: String,
        #[serde(rename = "type")]
        _type: String,
        allowed_origins: Vec<String>,
    },
}

impl Manifest {
    #[cfg(target_family = "unix")]
    pub fn new(browser: &Browser) -> Result<Self, failure::Error> {
        let path = current_exe()?.to_str().unwrap().to_string();
        Ok(match browser {
            &Browser::FireFox => Manifest::FireFox {
                name: "jp.rail44.octolo".to_string(),
                description: "Open files with local editor from GitHub web".to_string(),
                path,
                _type: "stdio".to_string(),
                allowed_extensions: vec!["{3de89a2b-180a-427e-85dd-29c983e93ba3}".to_string()],
            },
            &Browser::Chrome | Browser::Chromium => Manifest::Chrome {
                name: "jp.rail44.octolo".to_string(),
                description: "Open files with local editor from GitHub web".to_string(),
                path,
                _type: "stdio".to_string(),
                allowed_origins: vec![
                    "chrome-extension://igdmgdknajejkdpaonpnpjedakhppiob/".to_string()
                ],
            },
        })
    }
}

static FILE_NAME: &str = "jp.rail44.octolo.json";

pub fn manifest(browser_list: Vec<Browser>, write: bool) -> Result<(), failure::Error> {
    let home = home_dir().ok_or(crate::Error::CouldNotDetermineHomeDir)?;
    let manifest_list = browser_list
        .iter()
        .map(|browser| (browser, Manifest::new(browser)));
    for (browser, manifest) in manifest_list {
        let manifest = manifest?;
        if write {
            let path = home.join(browser.get_manifest_dir());
            create_dir_all(&path)?;
            let filename = path.join(FILE_NAME);
            let file = File::create(&filename)?;
            serde_json::to_writer_pretty(BufWriter::new(file), &manifest)?;
            println!(
                "Wrote manifest for {} to {}",
                browser,
                filename.to_str().unwrap()
            );
            continue;
        }
        println!("// manifest for {}", browser);
        serde_json::to_writer_pretty(stdout(), &manifest)?;
        println!("");
        println!("");
    }
    Ok(())
}
