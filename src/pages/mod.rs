// debug page
pub mod test;
pub mod login;

// normal pages
mod not_found;
mod home;
mod projects;
mod pvp;
mod tos;

pub use tos::*;
pub use pvp::*;
pub use not_found::*;
pub use home::*;
pub use projects::*;

// authourised pages
mod panel;
pub use panel::*;
