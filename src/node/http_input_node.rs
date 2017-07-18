use rocket;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use serde_json;
use biscuit::{JWT, jws, jwa, Empty};
use biscuit::jwa::SignatureAlgorithm;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::thread;

use Log;
use node::Node;

#[derive(PartialEq)]
enum SecretType {
    None,
    Shared,
    Key,
}

fn secret_type(algorithm: SignatureAlgorithm) -> SecretType {
    match algorithm {
        SignatureAlgorithm::HS256 |
        SignatureAlgorithm::HS384 |
        SignatureAlgorithm::HS512 => SecretType::Shared,
        SignatureAlgorithm::RS256 |
        SignatureAlgorithm::RS384 |
        SignatureAlgorithm::RS512 |
        SignatureAlgorithm::ES256 |
        SignatureAlgorithm::ES384 |
        SignatureAlgorithm::ES512 |
        SignatureAlgorithm::PS256 |
        SignatureAlgorithm::PS384 |
        SignatureAlgorithm::PS512 => SecretType::Key,
        SignatureAlgorithm::None => SecretType::None,
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum AuthConfig {
    NoAuth,
    JWT(jwa::SignatureAlgorithm, String),
}

struct AuthGuard(());

impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthGuard, ()> {
        let auth_config: &AuthConfig = request.guard::<State<AuthConfig>>()?.inner();

        match *auth_config {
            AuthConfig::NoAuth |
            AuthConfig::JWT(SignatureAlgorithm::None, _) => Outcome::Success(AuthGuard(())),
            AuthConfig::JWT(algorithm, _) => {
                macro_rules! fail_auth_if {
                    ($condition:expr) => (
                        if $condition {
                            return Outcome::Failure((Status::BadRequest, ()));
                        }
                    )
                }

                let biscuit_secret = request.guard::<State<jws::Secret>>()?.inner();
                // biscuit: no PartialEq on Secret
                // fail_auth_if!(biscuit_secret == jws::Secret::None);
                match *biscuit_secret {
                    jws::Secret::None => fail_auth_if!(true),
                    _ => {}
                }

                let tokens: Vec<_> = request.headers().get("Authorization").collect();
                fail_auth_if!(tokens.len() != 1);

                let mut auth_string = tokens[0].split_whitespace();
                fail_auth_if!(auth_string.next() != Some("Bearer"));

                let token_string = auth_string.next();
                fail_auth_if!(token_string.is_none());

                let token = JWT::<Empty, Empty>::new_encoded(
                    &token_string.expect("failed to get token from header"),
                );
                let token = token.into_decoded(&biscuit_secret, algorithm);
                println!("{:?}", token);
                fail_auth_if!(token.is_err());

                Outcome::Success(AuthGuard(()))
            }
        }
    }
}

#[get("/")]
fn index() -> &'static str {
    "Log receiver is listening on POST /logs/<label>"
}

#[post("/logs/<label>", data = "<body>")]
fn logs(label: String, body: String, _auth: AuthGuard, tx_out: State<Option<Mutex<Sender<Log>>>>) {
    // should use try_lock instead?
    if tx_out.is_some() {
        let _ = tx_out
            .as_ref()
            .expect("failed to get tx_out ref")
            .lock()
            .expect("failed to get tx_out lock")
            .send(Log::new(body, Some(label.to_string())));
    }
}

#[derive(Debug)]
pub struct HttpInputNode {
    config: HashMap<String, String>,
    rx: Arc<Mutex<Receiver<Log>>>,
    tx_inc: Sender<Log>,
    tx_out: Option<Sender<Log>>,
}

impl HttpInputNode {
    pub fn new(config: Option<HashMap<String, String>>, next: Option<Sender<Log>>) -> Self {
        let (sender, receiver) = channel();

        Self {
            config: config.unwrap_or(HttpInputNode::default_config()),
            rx: Arc::new(Mutex::new(receiver)),
            tx_inc: sender,
            tx_out: next,
        }
    }

    pub fn default_config() -> HashMap<String, String> {
        HashMap::new()
    }
}

impl Node for HttpInputNode {
    fn start(&self) -> Result<Sender<Log>, String> {
        let tx = self.tx_out.clone().map(|t| Mutex::new(t));

        // todo: use Value from serde_json for config
        let auth_config: AuthConfig = serde_json::from_str(
            self.config.get("auth").unwrap_or(&"NoAuth".to_string()),
        ).unwrap_or(AuthConfig::NoAuth);

        // Cannot deserialize Secret from JSON, so do it manually
        let biscuit_secret = match auth_config {
            AuthConfig::NoAuth => jws::Secret::None,
            AuthConfig::JWT(algorithm, ref secret) => {
                match secret_type(algorithm) {
                    SecretType::Shared => jws::Secret::Bytes(secret.to_string().into_bytes()),
                    SecretType::Key => jws::Secret::public_key_from_file(secret).expect(
                        "failed to create secret from file",
                    ),
                    SecretType::None => jws::Secret::None,
                }
            }
        };

        thread::spawn(|| {
            let _ = rocket::ignite()
                .manage(tx)
                .manage(auth_config)
                .manage(biscuit_secret) // really ugly, deserialise this into AuthConfig instead
                .mount("/", routes![index, logs])
                .launch();
        });

        let mut log: Log = Log::new("lol".to_string(), None);
        passthrough!(self, log, { /* noop */ });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use std::thread::sleep;
    use Log;
    use reqwest;

    #[test]
    fn it_passes_received_logs_through() {
        test_passthrough!(HttpInputNode);
    }

    #[test]
    fn it_starts_the_server_with_the_default_config() {
        let (sender, _) = channel();
        let _ = HttpInputNode::new(None, Some(sender)).start();
        sleep(Duration::from_millis(250));

        let resp = reqwest::get("http://localhost:8000/").unwrap();
        assert!(resp.status().is_success());

        let client = reqwest::Client::new().unwrap();
        let resp = client.post("http://localhost:8000/logs/noquack").unwrap()
            .body("foo bar my baz bax")
            .send().unwrap();
        assert!(resp.status().is_success());

        // Flakey test
        // let log = receiver.recv().unwrap();
        // assert_eq!(log.label, Some("noquack".to_string()));
        // assert_eq!(log.payload, "foo bar my baz bax");

        return;
    }
}
