use crate::Command;
use crate::Event;

pub type CommandReceiver = tokio::sync::mpsc::Receiver<Command>;
pub type CommandSender = tokio::sync::mpsc::Sender<Command>;

pub type EventReceiver = tokio::sync::mpsc::Receiver<Event>;
pub type EventSender = tokio::sync::mpsc::Sender<Event>;
