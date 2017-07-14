use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::fmt::Debug;
use std::collections::HashMap;

use log::Log;

#[cfg(test)]
#[macro_use]
pub mod testing;

// Can a closure be passed into the macro instead of a block?
// We wouldn't need the $log ident if we could instead move log into the user-defined closure
macro_rules! passthrough {
    ($sel:ident, $log:ident, $blk:block) => {
        let rx = $sel.rx.clone();
        let tx = $sel.tx_out.clone().map(|t| Arc::new(Mutex::new(t)));

        let _ = thread::spawn(move || {
            let tx_child = tx.clone();
            loop {
                $log = rx.lock().unwrap().recv().unwrap(); // panics on recv in test

                $blk

                if tx_child.as_ref().is_some() {
                    let _ = tx_child.as_ref().unwrap().lock().unwrap().send($log);
                }
            }
        });

        let opt: Result<Sender<Log>, String> = Ok($sel.tx_inc.clone());

        return opt
    }
}

pub mod http_input_node;
pub mod stdout_output_node;
pub mod postgres_output_node;
pub mod start_node;

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
}
