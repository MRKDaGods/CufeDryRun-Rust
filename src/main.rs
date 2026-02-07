// Hide console in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
mod desktop;

#[cfg(target_arch = "wasm32")]
mod web;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    desktop::run();

    #[cfg(target_arch = "wasm32")]
    web::run();
}
