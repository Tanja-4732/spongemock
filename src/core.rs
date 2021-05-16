use std::iter::FromIterator;

use rand::{thread_rng, Rng};

use crate::{Config, Spongemock};

// TODO this implementation is a disgrace; change that.
impl Spongemock for String {
    /// Transform a mutable str reference according to the set parameters in the specified options
    fn mock(&mut self, config: &Config) {
        // Abort on empty string
        if self.is_empty() {
            return;
        }

        // Construct a new RNG
        let mut rng = thread_rng();

        // A vector to store the new chars
        let mut chars = vec![];

        // Boolean flags if the previous character (and the one before it) was uppercase
        let mut prev_upper;
        let mut prev_prev_upper = true;

        // The transformation function
        let mut transform_char = |c: char, prob: f64, inverse: bool| {
            let gen_bool = rng.gen_bool(prob);

            let gen_bool = if inverse { !gen_bool } else { gen_bool };

            if gen_bool {
                chars.extend(c.to_uppercase());
            } else {
                chars.extend(c.to_lowercase());
            };

            gen_bool
        };

        // Transform the first char
        prev_upper = transform_char(self.chars().next().unwrap(), config.first_upper, false);

        // Transform the second char (if present)
        if self.len() != 1 {
            if prev_upper {
                prev_prev_upper = prev_upper;
                prev_upper =
                    transform_char(self.chars().nth(1).unwrap(), config.upper_to_lower, true);
            } else {
                prev_upper =
                    transform_char(self.chars().nth(1).unwrap(), config.lower_to_upper, false);
            }
        }

        for c in self.as_mut().chars().skip(2) {
            // Check if there might be three same-case letters in a row
            if prev_upper == prev_prev_upper {
                // Check if there are two lower- or two uppercase letters
                if prev_upper {
                    prev_prev_upper = prev_upper;
                    prev_upper = transform_char(c, config.upper_upper_to_lower, true);
                } else {
                    prev_prev_upper = prev_upper;
                    prev_upper = transform_char(c, config.lower_lower_to_upper, false);
                }
            } else if prev_upper {
                prev_prev_upper = prev_upper;
                prev_upper = transform_char(c, config.upper_to_lower, true);
            } else {
                prev_prev_upper = prev_upper;
                prev_upper = transform_char(c, config.lower_to_upper, false);
            }
        }

        *self = String::from_iter(chars);
    }
}

// impl<'a> Spongemock {
//     pub fn transform_str<T>(&self, s: T)
//     where
//         T: Into<&'a mut str>,
//     {
//         let s: &'a mut str = s.into();
//     }
// }
