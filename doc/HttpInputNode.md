# HttpInputNode

```json
{
  "node": "HttpInputNode",
  "conf": null,
  "next": null
}
```

## Config

### `address`, `port`, `log`

Ugly, but uses `Rocket.toml` in the root directory now.

[Rocket configuration options](https://rocket.rs/guide/configuration/)

### `auth`

Ugly, but right now it is a string of JSON.

```json
"auth": null
```

```json
"auth": "{\"JWT\":[\"RS256\",\"config/rsa_public_key.example.der\"]}"
```

#### Options

`null`, or an object that serialises to a `JWT` configuration object.
  
* `null` will disable authentication for the endpoint.

* `JWT(SignatureAlgorithm, shared_secret_or_der_file)`

  This is represented as a string of JSON (escaped) representing the object
  ```js
  { "JWT": [algorithm, shared_secret_or_der_file] }
  ```
  
  Example:
  ```
  "{\"JWT\":[\"RS256\",\"config/rsa_private_key.example.der\"]}"
  ```  

  If using HMAC, `shared_secret_or_der_file` is the shared secret.

  If using RSA/EC, `shared_secret_or_der_file` is the path to the `der` file.

  [List of supported algorithms](https://docs.rs/biscuit/0.0.6/biscuit/jwa/enum.SignatureAlgorithm.html)

#### Config/JWTs pairs for testing

##### RS256
```
"auth": "{\"JWT\":[\"RS256\",\"config/rsa_public_key.example.der\"]}"
```
```
eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL3d3dy5hY21lLmNvbS8iLCJzdWIiOiJKb2huIERvZSIsImF1ZCI6Imh0dHM6Ly9hY21lLWN1c3RvbWVyLmNvbS8iLCJuYmYiOjEyMzQsImNvbXBhbnkiOiJBQ01FIiwiZGVwYXJ0bWVudCI6IlRvaWxldCBDbGVhbmluZyJ9.THHNGg4AIq2RT30zecAD41is6j1ffGRn6GdK6cpl08esHufG5neJOMTO1fONVykOFgCaJw9jLP7GCdYumsMKU3434QAQyvLCPklHQWE7VcSFSdsf7skcvuvwPtkMWCGrzFK7seVv9OiJzjNzoeyS2d8io7wviFqkpcXwOVZW4ArP5katX4nIoXlwWfcK82E6MacSIL2uq_ha6yL2z7trq3dSszSnUevlWKq-9FIFk11XwToMTmGubkWyGk-k-dfHAXwnS1hADXkwSAemWoCG98v6zFtTZHOOAPnB09acEKVtVRFKZQa3V2IpdsHtRoPJU5pFgCXi8VRebHJm99yTXw
```

##### HS256

```
"auth": "{\"JWT\":[\"HS256\",\"secret\"]}"
```
```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.TJVA95OrM7E2cBab30RMHrHDcEfxjoYZgeFONFh7HgQ
```