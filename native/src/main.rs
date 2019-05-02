mod browser;
mod config;
mod neovim;
mod receive;

use browser::manifest;
use clap::{App, AppSettings, Arg, SubCommand};
use config::read_config;
use failure::Fail;
use receive::receive;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Could not get ghq.root")]
    CouldNotGetGhqRoot,
    #[fail(display = "Could not determine home dir")]
    CouldNotDetermineHomeDir,
    #[fail(display = "Could not find editor by requested id")]
    NotFoundEditor,
}

fn main() {
    let config = read_config().unwrap();
    let matches = App::new("octoro")
        .setting(AppSettings::AllowExternalSubcommands)
        .version("0.1.0")
        .about("Receive native messaging from browser to open local editor")
        .subcommand(
            SubCommand::with_name("manifest")
                .unset_setting(AppSettings::AllowExternalSubcommands)
                .about("Generate native messaging manifest for configured browser")
                .arg(
                    Arg::with_name("write")
                        .short("w")
                        .help("Write manifest insted of show"),
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .setting(AppSettings::SubcommandRequired)
                .unset_setting(AppSettings::AllowExternalSubcommands)
                .subcommand(
                    SubCommand::with_name("dump").about("Dump full config with optional fields"),
                ),
        )
        .get_matches();
    if let Some(c) = matches.subcommand_matches("manifest") {
        let write = c.is_present("write");
        manifest(config.browser_list, write).unwrap();
        return;
    }
    if let Some(c) = matches.subcommand_matches("config") {
        if c.subcommand_matches("dump").is_some() {
            println!("{}", &toml::to_string(&config).unwrap());
            return;
        }
    }
    receive(config).unwrap();
}
