#[cfg(not(target_arch = "wasm32"))]
mod desktop;

#[cfg(target_arch = "wasm32")]
mod web;

pub fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    desktop::run();

    #[cfg(target_arch = "wasm32")]
    web::run();
}
