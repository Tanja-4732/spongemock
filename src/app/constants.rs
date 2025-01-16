// The name of the application
pub const NAME: &str = "spongemock";

/// The main author of the application
pub const AUTHOR: &str = "Tanja <git@tanja.pw>";

/// The semantic-version string of the application
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Describes the application (i.e. its use cases) in a short phrase
pub const ABOUT: &str = "A CLI-tool for random text capitalization";

/// The licence notice (AGPL 3) of the application
pub const LICENSE: &str = concat![
    "Copyright 2021-2025 Tanja-4732; All rights reserved.\n",
    "Licensed under the AGPL 3.0 <https://www.gnu.org/licenses/agpl-3.0.en.html>"
];
