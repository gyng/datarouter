use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use Log;
use node::Node;

// TODO: Can this expand to a function?
macro_rules! test_passthrough {
    ($node:ident) => {
        use std::time::Duration;
        use node::testing::TestEchoNode;
        use std::sync::mpsc::channel;

        let (sender, receiver) = channel();
        let node = $node::new(None, Some(sender));

        let _ = TestEchoNode::new(
            "foobar".to_string(),
            Some("label".to_string()),
            node.start().ok(),
        ).start();

        let timeout = Duration::from_millis(100);
        let message: Log = receiver.recv_timeout(timeout).unwrap();

        assert_eq!(message.payload, "foobar".to_string());
        assert_eq!(message.label, Some("label".to_string()));
    }
}

#[derive(Debug)]
pub struct TestEchoNode {
    payload: String,
    label: Option<String>,
    tx_out: Option<Sender<Log>>,
}

impl TestEchoNode {
    pub fn new(payload: String, label: Option<String>, next: Option<Sender<Log>>) -> Self {
        Self {
            payload: payload,
            label: label,
            tx_out: next,
        }
    }
}

impl Node for TestEchoNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let (sender, _) = channel();
        let log = Log::new(self.payload.clone(), self.label.clone());
        let next = self.tx_out.clone().unwrap();
        let _ = next.send(log).unwrap();
        Ok(sender)
    }
}
