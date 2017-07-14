use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use Log;
use node::Node;

/// This Node blocks the main thread so the program does not terminate
#[derive(Debug)]
pub struct StartNode;

impl Node for StartNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let (sender, receiver) = channel();
        let _ = receiver.recv(); // block

        Ok(sender)
    }
}
