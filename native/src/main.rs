#![feature(custom_attribute)]

use clap::{value_t_or_exit, App, AppSettings, Arg, SubCommand};
use failure::Fail;

mod browser;
use browser::{manifest, Browser};

mod receive;
use receive::receive;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Could not get ghq.root")]
    CouldNotGetGhqRoot,
    #[fail(display = "Could not determine home dir")]
    CouldNotDetermineHomeDir,
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
