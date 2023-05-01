use crate::constants;
use anyhow::anyhow;
use clap::{App, Arg, ArgMatches};
use spongemock::Config;

#[derive(Debug)]
pub struct CliOptions {
    pub text: String,
    pub config: Config,
    pub no_newline: bool,
    pub quiet: bool,
}

pub fn configure_parser() -> App<'static, 'static> {
    let app = App::new(constants::NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about(constants::ABOUT)
        .after_help(constants::LICENSE)
        .args(&[
            Arg::with_name("text")
                .help("The text to be processed")
                .index(1)
                .min_values(1),
            Arg::with_name("first_upper")
                .takes_value(true)
                .help("The chance of the first letter being in uppercase")
                .short("f")
                .long("first-upper")
                .value_name("0..1"),
            Arg::with_name("lower_to_upper")
                .takes_value(true)
                .help("The chance of a character to be uppercase, if the previous character is lowercase")
                .short("l")
                .long("lower-to-upper")
                .value_name("0..1"),
            Arg::with_name("upper_to_lower")
                .takes_value(true)
                .help("The chance of a character to be lowercase, if the previous character is uppercase")
                .short("u")
                .long("upper-to-lower")
                .value_name("0..1"),
            Arg::with_name("upper_upper_to_lower")
                .takes_value(true)
                .help("The chance of a character to be lowercase, if the previous two characters are uppercase")
                .short("U")
                .long("upper-upper-to-lower")
                .value_name("0..1"),
            Arg::with_name("lower_lower_to_upper")
                .takes_value(true)
                .help("The chance of a character to be uppercase, if the previous two characters are lowercase")
                .short("L")
                .long("lower-lower-to-upper")
                .value_name("0..1"),
                Arg::with_name("no-quiet")
                .takes_value(false)
                .help("suppress author and license notice")
                .short("q")
                .long("no-quiet"),
            Arg::with_name("no_newline")
                .takes_value(false)
                .help("If the newline should be suppressed")
                .short("n")
                .long("no-newline")
        ]);

    app
}

pub fn get_options(matches: &ArgMatches) -> Result<CliOptions, anyhow::Error> {
    let parse = |s: &str| s.parse::<f64>();

    Ok(CliOptions {
        text: matches
            .values_of("text")
            .ok_or(anyhow!("No text specified"))?
            .collect::<Vec<_>>()
            .join(" "),
        no_newline: matches.is_present("no_newline"),
        config: Config {
            first_upper: matches
                .value_of("first_upper")
                .map(parse)
                .unwrap_or(Ok(Config::default().first_upper))?,
            lower_to_upper: matches
                .value_of("lower_to_upper")
                .map(parse)
                .unwrap_or(Ok(Config::default().lower_to_upper))?,
            upper_to_lower: matches
                .value_of("upper_to_lower")
                .map(parse)
                .unwrap_or(Ok(Config::default().upper_to_lower))?,
            upper_upper_to_lower: matches
                .value_of("upper_upper_to_lower")
                .map(parse)
                .unwrap_or(Ok(Config::default().upper_upper_to_lower))?,
            lower_lower_to_upper: matches
                .value_of("lower_lower_to_upper")
                .map(parse)
                .unwrap_or(Ok(Config::default().lower_lower_to_upper))?,
        },
        quiet: matches.is_present("no-quiet"),
    })
}
