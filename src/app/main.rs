#![warn(clippy::pedantic)]

mod cli;
mod constants;

use spongemock::Spongemock;

fn main() -> Result<(), anyhow::Error> {
    // Parse the command line parameters into arg-matches
    let matches = cli::configure_parser().get_matches();

    // Print the name and version of the application along its license notice
    if matches.is_present("no-quiet") {
        eprintln!("{} {}", constants::NAME, constants::VERSION);
        eprintln!("{}\n", constants::LICENSE);
    }

    // Try to extract the desired configuration from the arg-matches
    let mut cli_options = cli::get_options(&matches)?;

    // Transform the text
    cli_options.text.mock(&cli_options.config);

    // Print the result to stdout
    if cli_options.no_newline {
        print!("{}", cli_options.text);
    } else {
        println!("{}", cli_options.text);
    }

    Ok(())
}
