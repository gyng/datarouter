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
        let tx = self.tx_out.clone().map(|t| Arc::new(Mutex::new(t)));

        let _ = thread::spawn(move || {
            let tx_child = tx.clone();
            loop {
                let log = receiver.lock().unwrap().recv().unwrap();

                println!("{:?}", log.clone());

                if tx_child.as_ref().is_some() {
                    let m = tx_child.as_ref().unwrap();
                    let _ = m.lock().unwrap().send(log);
                }
            }
        });

        Ok(self.tx_inc.clone())
    }
}
