mod app;
mod config;

fn main() -> Result<(), miette::Error> {
    tracing_subscriber::fmt::init();
    crate::app::boot()
}
