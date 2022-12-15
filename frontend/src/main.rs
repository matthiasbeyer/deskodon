mod message;
mod tauri;
mod view;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    println!("Starting app");
    yew::Renderer::<crate::view::App>::new().render();
}
