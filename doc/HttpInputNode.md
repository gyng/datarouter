# HttpInputNode

Listens to `/log/<label>` for incoming POSTs and forwards the request body

## Config

```json
{
  "node": "HttpInputNode",
  "conf": null,
  "next": null
}
```

### `address`, `port`, `log`

For now, uses Rocket configuration settings (ENV variables or `Rocket.toml`)

[Rocket configuration options](https://api.rocket.rs/rocket/config/)

### `auth`

Without authentication:

```json
"auth": null
```

With JWT (example):

```json
"auth": {
  "algorithm": "RS256",
  "secret_sauce": "config/rsa_public_key.example.der"
}
```

#### Options

`null`, or an object that serialises to a JWT configuration object

* `null` will disable authentication for the endpoint

* An object with `algorithm` and `secret_sauce` keys will enable JWT authentication

  * `algorithm`: a valid JWT algorithm
  * `secret_sauce`: shared secret for HMAC, or filepath to a `.der` for algorithms using a key

  RSA256 example
  ```json
  "auth": {
    "algorithm": "RS256",
    "secret_sauce": "config/rsa_public_key.example.der"
  }
  ```

  HMAC example
  ```json
  "auth": {
    "algorithm": "HS256",
    "secret_sauce": "secret"
  }
  ```

  [List of supported algorithms](https://docs.rs/biscuit/0.0.6/biscuit/jwa/enum.SignatureAlgorithm.html)

#### Config/JWTs pairs for testing

##### RS256

```json
"auth": {
  "algorithm": "RS256",
  "secret_sauce": "config/rsa_public_key.example.der"
}
```

```
eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL3d3dy5hY21lLmNvbS8iLCJzdWIiOiJKb2huIERvZSIsImF1ZCI6Imh0dHM6Ly9hY21lLWN1c3RvbWVyLmNvbS8iLCJuYmYiOjEyMzQsImNvbXBhbnkiOiJBQ01FIiwiZGVwYXJ0bWVudCI6IlRvaWxldCBDbGVhbmluZyJ9.THHNGg4AIq2RT30zecAD41is6j1ffGRn6GdK6cpl08esHufG5neJOMTO1fONVykOFgCaJw9jLP7GCdYumsMKU3434QAQyvLCPklHQWE7VcSFSdsf7skcvuvwPtkMWCGrzFK7seVv9OiJzjNzoeyS2d8io7wviFqkpcXwOVZW4ArP5katX4nIoXlwWfcK82E6MacSIL2uq_ha6yL2z7trq3dSszSnUevlWKq-9FIFk11XwToMTmGubkWyGk-k-dfHAXwnS1hADXkwSAemWoCG98v6zFtTZHOOAPnB09acEKVtVRFKZQa3V2IpdsHtRoPJU5pFgCXi8VRebHJm99yTXw
```

##### HS256

```json
"auth": {
  "algorithm": "HS256",
  "secret_sauce": "secret"
}
```

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ
```