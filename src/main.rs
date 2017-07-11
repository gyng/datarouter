#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::fmt::Debug;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct Log {
    timestamp: u32,
    payload: String,
    label: Option<String>,
}

impl Log {
    fn new(payload: String, label: Option<String>) -> Log {
        Log {
            timestamp: 0,
            payload: payload,
            label: label,
        }
    }
}

trait Node: Debug {
    fn start(&self) -> Result<Sender<Log>, String> {
        let (sender, _receiver) = channel();
        Ok(sender)
    }

    fn stop(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
struct HttpInputNode {
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl HttpInputNode {
    fn new(_config: &String, next: Option<Sender<Log>>) -> Self {
        let (sender, receiver) = channel();

        Self {
            rx: Arc::new(Mutex::new(receiver)),
            tx_inc: sender,
            tx_out: next,
        }
    }
}

mod http_input_node {
    use rocket::State;
    use super::Log;
    use std::sync::mpsc::Sender;
    use std::sync::Mutex;

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
}

impl Node for HttpInputNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let tx: Mutex<Sender<Log>> = Mutex::new(self.tx_out.clone().unwrap());

        rocket::ignite()
            .manage(tx)
            .mount("/", routes![http_input_node::index, http_input_node::logs])
            .launch();

        Ok(self.tx_inc.clone())
    }
}

#[derive(Debug)]
struct StdoutOutputNode {
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl StdoutOutputNode {
    fn new(_config: &String, next: Option<Sender<Log>>) -> Self {
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

fn main() {
    let _ = HttpInputNode::new(
        &"".to_string(),
        StdoutOutputNode::new(&"".to_string(), None).start().ok(),
    ).start()
        .unwrap();
}
