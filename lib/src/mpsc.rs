use crate::Event;

pub type EventReceiver = tokio::sync::mpsc::Receiver<Event>;
pub type EventSender = tokio::sync::mpsc::Sender<Event>;
