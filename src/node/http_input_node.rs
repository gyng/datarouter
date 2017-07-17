use rocket;
use rocket::State;
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use serde_json;
use biscuit::{self, JWT, jws, jwa, Empty};
use biscuit::jwa::SignatureAlgorithm;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::thread;

use Log;
use node::Node;

#[derive(Serialize, Deserialize, Debug)]
enum AuthConfig {
    NoAuth,
    JWT(biscuit::jwa::SignatureAlgorithm, String),
}

struct AuthGuard(bool);

impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthGuard, ()> {
        let auth_config: &AuthConfig = request.guard::<State<AuthConfig>>()?.inner();

        match *auth_config {
            AuthConfig::NoAuth => Outcome::Success(AuthGuard(true)),
            AuthConfig::JWT(SignatureAlgorithm::HS256, ref secret) |
            AuthConfig::JWT(SignatureAlgorithm::HS384, ref secret) |
            AuthConfig::JWT(SignatureAlgorithm::HS512, ref secret) => {
                macro_rules! fail_auth_if {
                    ($condition:expr) => (
                        if $condition {
                            return Outcome::Failure((Status::BadRequest, ()));
                        }
                    )
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
                let biscuit_secret = jws::Secret::Bytes(secret.to_string().into_bytes());
                let token = token.into_decoded(&biscuit_secret, jwa::SignatureAlgorithm::HS256);
                fail_auth_if!(token.is_err());

                Outcome::Success(AuthGuard(true))
            }
            AuthConfig::JWT(_, ref _secret) => {
                println!("JWT algorithm not implemented yet");
                return Outcome::Failure((Status::BadRequest, ()));
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

        // todo: use Value from serde_json
        let auth_algorithm: AuthConfig = serde_json::from_str(
            self.config.get("auth").unwrap_or(&"NoAuth".to_string()),
        ).unwrap_or(AuthConfig::NoAuth);

        println!("{:?}", auth_algorithm);
        let auth_config = auth_algorithm;

        thread::spawn(|| {
            let _ = rocket::ignite()
                .manage(tx)
                .manage(auth_config)
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

    #[test]
    fn it_passes_received_logs_through() {
        test_passthrough!(HttpInputNode);
    }
}
