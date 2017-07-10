#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

trait Node {
    fn start(&self) -> Result<(), String>;
    fn is_input(&self) -> bool;
    fn chain(&self, node: Box<Node>) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    // fn process(&self) -> Result<(), String>;
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

    #[post("/logs/<label>", format="application/json")]
    fn logs(label: &str) {
        
    }
}

impl HttpInputNode {
    fn new(config: &String) -> HttpInputNode {
        HttpInputNode { chained: vec!(), rocket: None }
    }
}

impl Node for HttpInputNode {
    fn start(&self) -> Result<(), String> {
        rocket::ignite().mount("/", routes![HttpInput::index, HttpInput::logs]).launch();
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
}
