#![warn(clippy::pedantic)]

use spongemock::{Config, Spongemock};

fn main() {
    // TODO implement CLI interaction using clap
    demo();
}

fn demo() {
    let config = Config::default();

    for _ in 0..10 {
        let mut mockery = String::from("Hello, world! Greetings from the hard-coded string!");
        mockery.mock(&config);

        println!("{}", mockery);
    }
}
