use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use Log;
use node::Node;

#[derive(Debug)]
pub struct StdoutOutputNode {
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl StdoutOutputNode {
    pub fn new(_config: Option<&HashMap<String, String>>, next: Option<Sender<Log>>) -> Self {
        let (sender, receiver) = channel();

        Self {
            rx: Arc::new(Mutex::new(receiver)),
            tx_inc: sender,
            tx_out: next,
        }
    }
}

impl Node for StdoutOutputNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let receiver = self.rx.clone();

        let _ = thread::spawn(move || loop {
            println!("{:?}", receiver.lock().unwrap().recv().unwrap());
        });

        Ok(self.tx_inc.clone())
    }
}
