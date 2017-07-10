#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

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

trait Node {
    fn start(&self) -> Result<(), String>;
    fn is_input(&self) -> bool;
    fn chain(&self, node: Box<Node>) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    // Should encapsulate chaining logic?
    // fn send(&self, data: Log) -> 
    fn process(&self, data: Log) -> Result<(), String> {
        println!("{:?}", data);
        Ok(())
    }
    // fn chained(&self) -> Vec<Box<Node>>;
    // fn forward(&self) -> Result<(), String>;
}

struct HttpInputNode {
    chained: Vec<Box<Node>>,
    rocket: Option<rocket::Rocket>,
}

mod HttpInput {
    #[get("/")]
    fn index() -> &'static str {
        "Hello, world!"
    }

    // replace with post, check jwt
    #[get("/logs/<label>", format="application/json")]
    fn logs(label: &str) {
        ????process(super::Log::new(label.to_string(), Some("HELLO".to_string())))
    }
}

impl HttpInputNode {
    fn new(config: &String) -> HttpInputNode {
        HttpInputNode { chained: vec!(), rocket: None }
    }
}

impl Node for HttpInputNode {
    fn start(&self) -> Result<(), String> {
        rocket::ignite()
            .manage(self)
            .mount("/", routes![HttpInput::index, HttpInput::logs]).launch();
        Ok(())
    }

    fn is_input(&self) -> bool {
        true
    }

    fn chain(&self, node: Box<Node>) -> Result<(), String> {
        Ok(())
    }

    fn stop(&self) -> Result<(), String> {
        Ok(())
    }
}

struct PostgresOutputNode {
    chained: Vec<Box<Node>>,
}

fn main() {
    let rooter = HttpInputNode::new(&"".to_string());
    rooter.start();
}
