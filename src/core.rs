use std::{convert::TryFrom, iter::FromIterator, mem};

use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};

use crate::{Config, Spongemock};

impl Spongemock for String {
    /// Transform a mutable str reference according to the set parameters in the specified options
    fn mock(&mut self, config: &Config) {
        let mut rng = thread_rng();

        let mut chars = vec![];

        for c in self.as_mut().chars() {
            if rng.gen_bool(config.first_upper) {
                chars.extend(c.to_uppercase())
            } else {
                chars.extend(c.to_lowercase())
            };
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
