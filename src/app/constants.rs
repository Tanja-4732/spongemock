// The name of the application
pub const NAME: &'static str = "spongemock";

/// The main author of the application
pub const AUTHOR: &'static str = "Tanja <git@tanja.pw>";

/// The semantic-version string of the application
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Describes the application (i.e. its use cases) in a short phrase
pub const ABOUT: &'static str = "A CLI-tool for random text capitalization";

/// The licence notice (AGPL 3) of the application
pub const LICENSE: &'static str = concat![
    "Copyright 2021-2023 Tanja-4732; All rights reserved.\n",
    "Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>"
];
