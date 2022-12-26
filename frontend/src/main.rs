use seed::{prelude::wasm_bindgen, App};

mod message;
mod model;
mod tauri;
mod view;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    println!("Starting app");

    App::start(
        "deskodon",
        crate::model::Model::init,
        crate::model::update,
        crate::view::view,
    );
}
