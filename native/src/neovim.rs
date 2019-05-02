use crate::receive::OpenMessage;
use neovim_lib::{Neovim, NeovimApi, Session};
use serde_json::{json, Value};
use std::path::PathBuf;

pub fn open(
    address: &str,
    work_dir: &PathBuf,
    message: &OpenMessage,
) -> Result<Value, failure::Error> {
    let mut session = Session::new_unix_socket(address)?;
    session.start_event_loop();
    let mut neovim = Neovim::new(session);
    neovim.command(&format!("cd {}", work_dir.to_str().unwrap()))?;
    neovim.command(&format!("tabedit {}", message.path))?;
    neovim.input(&format!("{}G", message.line))?;
    Ok(json!({}))
}
