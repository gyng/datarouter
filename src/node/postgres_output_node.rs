use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use postgres::{Connection, TlsMode};

use Log;
use node::Node;

#[derive(Debug)]
pub struct PostgresOutputNode {
    config: Option<HashMap<String, String>>,
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl PostgresOutputNode {
    pub fn new(config: Option<HashMap<String, String>>, next: Option<Sender<Log>>) -> Self {
        let (sender, receiver) = channel();

        Self {
            config: config,
            rx: Arc::new(Mutex::new(receiver)),
            tx_inc: sender,
            tx_out: next,
        }
    }
}

impl Node for PostgresOutputNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        // TODO: Add sane defaults
        let config = self.config.clone().unwrap();
        let table_name = config.get("table_name").unwrap().to_string();
        let db_address = config.get("connection").unwrap().to_string();

        let conn = Connection::connect(db_address, TlsMode::None).unwrap();

        // TODO: Support JSONB + serde
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {:?} (
                id SERIAL PRIMARY KEY,
                rfc3339 TIMESTAMP WITH TIME ZONE,
                body VARCHAR NOT NULL
            )",
                &table_name
            ),
            &[],
        ).unwrap();

        let receiver = self.rx.clone();
        let tx = self.tx_out.clone().map(|t| Arc::new(Mutex::new(t)));

        let _ = thread::spawn(move || {
            let tx_child = tx.clone();

            loop {
                let log = receiver.lock().unwrap().recv().unwrap();

                let _ = conn.execute(
                    &format!(
                        "INSERT INTO {:?} (rfc3339, body) VALUES ($1, $2)",
                        table_name
                    ),
                    &[&log.timestamp, &log.payload],
                ).map_err(|err| println!("PostgresOutputNode error: {:?}", err));

                if tx_child.as_ref().is_some() {
                    let _ = tx_child.as_ref().unwrap().lock().unwrap().send(log);
                }
            }
        });

        Ok(self.tx_inc.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_passes_received_logs_through() {
        test_passthrough!(PostgresOutputNode);
    }
}
