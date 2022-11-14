mod config;
mod ui;

fn main() -> Result<(), miette::Error> {
    tracing_subscriber::fmt::init();
    crate::ui::boot()
}
