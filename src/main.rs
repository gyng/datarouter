#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::Sender;

mod log;
mod node;

use log::Log;
use node::{NodeType, NodeConfig};
use node::http_input_node::HttpInputNode;
use node::stdout_output_node::StdoutOutputNode;
use node::Node;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pipeline_path = &args[1];

    let mut config_json = String::new();
    File::open(pipeline_path)
        .map_err(|e| format!("{:?}", e))
        .unwrap()
        .read_to_string(&mut config_json)
        .map_err(|e| format!("{:?}", e))
        .unwrap();

    let config: NodeConfig = serde_json::from_str(config_json.as_ref())
        .map_err(|e| format!("{:?}", e))
        .unwrap();

    start_pipeline(&config);
}

fn start_pipeline(node_config: &NodeConfig) -> Option<Sender<Log>> {
    let next = if let Some(ref next_config) = node_config.next {
        start_pipeline(&next_config)
    } else {
        None
    };

    match node_config.node {
        NodeType::StdoutOutputNode => StdoutOutputNode::new(&node_config.conf, next).start().ok(),
        NodeType::HttpInputNode => HttpInputNode::new(&node_config.conf, next).start().ok(),
    }
}
