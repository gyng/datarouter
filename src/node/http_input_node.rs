use rocket;
use rocket::State;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;

use Log;
use node::Node;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/logs/<label>")]
fn logs(label: &str, sender: State<Mutex<Sender<Log>>>) {
    // should use try_lock instead?
    let _ = sender.inner().lock().unwrap().send(Log::new(
        "lol".to_string(),
        Some(label.to_string()),
    ));
}

#[derive(Debug)]
pub struct HttpInputNode {
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl HttpInputNode {
    pub fn new(_config: &HashMap<String, String>, next: Option<Sender<Log>>) -> Self {
        let (sender, receiver) = channel();

        Self {
            rx: Arc::new(Mutex::new(receiver)),
            tx_inc: sender,
            tx_out: next,
        }
    }
}

impl Node for HttpInputNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let tx: Mutex<Sender<Log>> = Mutex::new(self.tx_out.clone().unwrap());

        rocket::ignite()
            .manage(tx)
            .mount("/", routes![index, logs])
            .launch();

        Ok(self.tx_inc.clone())
    }
}
