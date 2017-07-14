# datarouter

[![Build Status](https://travis-ci.org/gyng/rlogdata.svg?branch=master)](https://travis-ci.org/gyng/rlogdata)

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
