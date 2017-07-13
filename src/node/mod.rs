use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::fmt::Debug;
use std::collections::HashMap;

use log::Log;

pub mod http_input_node;
pub mod stdout_output_node;
pub mod postgres_output_node;

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeType {
    HttpInputNode,
    StdoutOutputNode,
    PostgresOutputNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node: NodeType,
    pub conf: Option<HashMap<String, String>>,
    pub next: Option<Box<NodeConfig>>,
}

pub trait Node: Debug {
    fn start(&self) -> Result<Sender<Log>, String> {
        let (sender, _receiver) = channel();
        Ok(sender)
    }

    fn stop(&self) -> Result<(), String> {
        Ok(())
    }
}
