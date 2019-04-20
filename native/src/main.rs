#![feature(custom_attribute)]

mod browser;
mod config;
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
    #[fail(display = "Could not determine config dir")]
    CouldNotDetermineConfigDir,
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
                .about("Generate and place native manifest")
                .arg(
                    Arg::with_name("write")
                        .short("w")
                        .help("Write manifest insted of show"),
                ),
        )
        .get_matches();
    if let Some(c) = matches.subcommand_matches("manifest") {
        let write = c.is_present("write");
        manifest(config.browser, write).unwrap();
        return;
    }
    receive(config).unwrap();
}
