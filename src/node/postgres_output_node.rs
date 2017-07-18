use serde_json::{Map, Value};

use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

use postgres::{Connection, TlsMode};

use Log;
use node::Node;

#[derive(Debug)]
pub struct PostgresOutputNode {
    config: Map<String, Value>,
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl PostgresOutputNode {
    pub fn new(config: Option<Value>, next: Option<Sender<Log>>) -> Self {
        let (sender, receiver) = channel();

        Self {
            config: config
                .unwrap_or(PostgresOutputNode::default_config())
                .as_object()
                .unwrap()
                .clone(),
            rx: Arc::new(Mutex::new(receiver)),
            tx_inc: sender,
            tx_out: next,
        }
    }

    pub fn default_config() -> Value {
        json!({
            "table_name": "logs",
            "connection": "postgres://localhost:5432"
        })
    }
}

impl Node for PostgresOutputNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let table_name = self.config
            .get("table_name")
            .expect("missing table_name key for PG configuration")
            .as_str()
            .expect("`table_name` is not a valid string")
            .to_string();
        let db_address = self.config
            .get("connection")
            .expect("missing connection key for PG configuration")
            .as_str()
            .expect("`connection` is not a valid string")
            .to_string();

        let conn = Connection::connect(db_address, TlsMode::None)
            .map_err(|err| println!("PostgresOutputNode error: {:?}", err))
            .and_then(|c| {
                let _ = c.execute(
                    &format!(
                        "CREATE TABLE IF NOT EXISTS {:?} (
                        id SERIAL PRIMARY KEY,
                        rfc3339 TIMESTAMP WITH TIME ZONE,
                        body VARCHAR NOT NULL
                    )",
                        &table_name
                    ),
                    &[],
                ).map_err(|err| println!("PostgresOutputNode error: {:?}", err));

                Ok(c)
            });

        // // TODO: Support JSONB + serde
        let mut log: Log = Log::empty();
        passthrough!(self, log, {
            if let Ok(ref c) = conn {
                let _ = c.execute(
                    &format!(
                        "INSERT INTO {:?} (rfc3339, body) VALUES ($1, $2)",
                        table_name
                    ),
                    &[&log.timestamp, &log.payload],
                ).map_err(|err| println!("PostgresOutputNode error: {:?}", err));
            }
        });
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
