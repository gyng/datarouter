#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::fmt::Debug;

#[derive(Debug)]
struct Log {
    timestamp: u32,
    payload: String,
    label: Option<String>,
}

impl Log {
    fn new(payload: String, label: Option<String>) -> Log {
        Log { timestamp: 0, payload: payload, label: label}
    }
}

trait Node: Send + Sync + Debug {
    fn start(self) -> Result<(), String> where Self: std::marker::Sized {
        self.start_next()
    }

    fn start_next(&self) -> Result<(), String> where Self: std::marker::Sized {
        if let &Some(ref next) = self.get_next() {
            return next.start();
        }

        Ok(())
    }

    fn stop(&self) -> Result<(), String> {
        Ok(())
    }

    fn process(&self, data: Log) -> Result<(), String> {
        // passthrough
        self.process_next(data)
    }

    fn process_next(&self, data: Log) -> Result<(), String> {
        if let &Some(ref next) = self.get_next() {
            return next.process(data);
        }

        Ok(())
    }

    fn get_next(&self) -> &Option<Box<Node>>;
}

#[derive(Debug)]
struct HttpInputNode {
    next: Option<Box<Node>>,
}

mod http_input_node {
    use rocket::State;
    use super::{Log, Node};

    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    #[get("/logs/<label>")]
    fn logs(label: &str, node: State<Box<Node>>) {
        println!("{:?}", node);
        let _ = node.process(Log::new("lol".to_string(), Some(label.to_string())));
    }
}

impl HttpInputNode {
    fn new(config: &String, next: Option<Box<Node>>) -> Self {
        Self { next: next }
    }
}

impl Node for HttpInputNode {
    fn start(self) -> Result<(), String> {
        let node: Box<Node> = Box::new(self);

        rocket::ignite()
            .manage(node)
            .mount("/", routes![http_input_node::index, http_input_node::logs]).launch();

        self.start_next()
    }

    fn get_next(&self) -> &Option<Box<Node>> {
        &self.next
    }
}

#[derive(Debug)]
struct StdoutOutputNode {
    next: Option<Box<Node>>,
}

impl StdoutOutputNode {
    fn new(config: &String, next: Option<Box<Node>>) -> Self {
        Self { next: next }
    }
}

impl Node for StdoutOutputNode {
    fn get_next(&self) -> &Option<Box<Node>> {
        &self.next
    }

    fn process(&self, data: Log) -> Result<(), String> {
        println!("{:?}", data);
        self.process_next(data)
    }
}

fn main() {
    let _ = HttpInputNode::new(&"".to_string(),
        Some(Box::new(StdoutOutputNode::new(&"".to_string(), None)))
    ).start();
}
