# PostgresOutputNode

Writes logs to a postgres table with the schema

```sql
id      SERIAL PRIMARY KEY
rfc3339 TIMESTAMP WITH TIME ZONE
body    VARCHAR NOT NULL
```

Will create a new table if it does not exist on startup.


## Config

```json
"node": "PostgresOutputNode",
"conf": {
    "connection": "postgres://user:pass@localhost:5432/dbname",
    "table_name": "my_table_name"
},
"next": null
```

### `connection`

The Postgres connection URI.

See: [Connection Strings](https://www.postgresql.org/docs/current/static/libpq-connect.html#LIBPQ-CONNSTRING)

### `table_name`

The name of the table to store the logs in.