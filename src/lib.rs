#[cfg(test)]
mod test;

pub struct Spongemock {
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
}

impl Default for Spongemock {
    fn default() -> Self {
        Self {
            first_upper: 0.5,
            lower_to_upper: 0.5,
            upper_to_lower: 0.5,
            upper_upper_to_lower: 0.75,
            lower_lower_to_upper: 0.75,
        }
    }
}

impl Spongemock {
    pub fn transform_str<T>(&self, s: T)
    where
        T: Into<String>,
    {
        let s = s.into();
    }
}
