# datarouter

[![Build Status](https://travis-ci.org/gyng/datarouter.svg?branch=master)](https://travis-ci.org/gyng/datarouter)

In development. Intended to route logs/data from one place to another, with processing as needed. Maybe a simpler version of rsyslog?

## Usage

For pipeline configuration see `pipeline.example.json`

```
cargo run --release -- config/pipeline.example.json
```

## TODO 

* Postgres JSONB w/ serde support, sane default config
* Move Rocket config into pipeline.json
* Add auth for HTTP input node (JWT, Basic, None)
* Change config HashMap<String, String> to <String, Value>
* Fix panics in cargo test
* Move key reading out from request guard for certificate-using JWT algorithms
