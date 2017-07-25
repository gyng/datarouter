# datarouter

[![Build Status](https://travis-ci.org/gyng/datarouter.svg?branch=master)](https://travis-ci.org/gyng/datarouter)

In development. Intended to route logs/data from one place to another, with processing as needed. A simpler version of rsyslog?

## Basic features

* Straightforward JSON pipeline definition and configuration
* HTTP endpoint for receiving data, with optional JWT
* Postgres writer

## Usage

```
cargo run --release -- config/pipeline.example.json
```

For pipeline configuration see [`config/pipeline.example.json`](config/pipeline.example.json)

For individual node configuration optinos, see [`doc/`](doc/)

A Docker image and example Compose configuration are provided for convenience. It will use `config/pipeline.json` as the configuration file.

```
cp docker-compose.override-example.yml docker-compose.override.yml

docker-compose up
```

To build an executable:

```
cargo build --release
```

## TODO

* Move Rocket config into `pipeline.json`
* Add basic auth for HTTP input node?
* CORS
* Elasticsearch output?
* File input and output
* STDIN input?
