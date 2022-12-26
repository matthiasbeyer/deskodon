mod tauri;

use seed::prelude::*;
use seed::div;
use seed::C;
use seed::button;


struct Model {
    counter: i32,
}

enum Msg {
    Increment,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

use seed::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Starting app");
    println!("Starting app");
    App::start("app", init, update, view);
}
