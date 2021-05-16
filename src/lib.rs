#[cfg(test)]
mod test;

mod core;

pub struct Config {
    /// The chance of the first letter being in uppercase
    pub first_upper: f64,

    /// The chance of a character to be uppercase, if the previous character is lowercase
    pub lower_to_upper: f64,

    /// The chance of a character to be lowercase, if the previous character is uppercase
    pub upper_to_lower: f64,

    /// The chance of a character to be lowercase, if the previous two characters are uppercase
    pub upper_upper_to_lower: f64,

    /// The chance of a character to be uppercase, if the previous two characters are lowercase
    pub lower_lower_to_upper: f64,
    // /// The random number generator
    // rng: ThreadRng,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            first_upper: 0.5,
            lower_to_upper: 0.5,
            upper_to_lower: 0.5,
            upper_upper_to_lower: 0.75,
            lower_lower_to_upper: 0.75,
            // rng: rand::thread_rng(),
        }
    }
}

pub trait Spongemock {
    /// Transform a mutable Self reference according to the set parameters in the specified options
    fn mock(&mut self, config: &Config);

    /// Use the default configuration for the mock implementation
    fn mock_default(&mut self) {
        self.mock(&Config::default())
    }
}
