# \<Untitled project\>

In development. Intended to route logs/data from one place to another, with processing as needed. Maybe a simpler version of rsyslog?

## Usage

For pipeline configuration see `pipeline.json`

```
cargo run --release -- pipeline.json
```

## TODO 

* Postgres JSONB w/ serde support, sane default config
* Move Rocket config into pipeline.json
* Add auth for HTTP input node (JWT, Basic, None)
