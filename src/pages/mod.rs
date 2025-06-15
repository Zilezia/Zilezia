pub mod test;
pub mod login;
pub mod template;

// normal pages
mod pvp;
pub use pvp::*;
mod tos;
pub use tos::*;
mod auth;
pub use auth::*;
mod home;
pub use home::*;
mod projects;
pub use projects::*;
mod not_found;
pub use not_found::*;

// authourised pages
mod panel;
pub use panel::*;
