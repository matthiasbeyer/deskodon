use seed::App;

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
        update,
        crate::view::view,
    );
}

fn update(
    msg: message::Message,
    model: &mut model::Model,
    orders: &mut impl seed::prelude::Orders<message::Message>,
) {
    model.update(msg, orders)
}
