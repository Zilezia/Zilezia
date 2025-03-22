pub mod db;
pub mod api;
pub mod app;
pub mod error;
pub mod state;
pub mod models;
pub mod router;
pub mod handler;

pub mod user;

pub mod pages;
pub mod components;

pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
