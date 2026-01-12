#![warn(clippy::pedantic)]
#![deny(clippy::disallowed_methods)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod voice;

mod join;
use std::sync::LazyLock;

pub use join::join;

mod test;
use reqwest::Client;
pub use test::test;

mod leave;
pub use leave::leave;

mod cat;
pub use cat::cat;

mod edit_snipe;
pub use edit_snipe::edit_snipe;

mod snipe;
pub use snipe::snipe;

mod get_song;
pub use get_song::get_song;

mod dog;
pub use dog::dog;

pub static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
