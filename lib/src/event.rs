#[derive(Debug)]
pub enum Event {
    Login {
        instance: String,
        username: String,
    }
}
