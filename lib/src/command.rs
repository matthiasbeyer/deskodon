/// A command gets send from the frontend to the backend
#[derive(Debug)]
pub enum Command {
    QuitApp,
    PostText { text: String },
}
