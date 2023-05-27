# sqlx-transparent-json-decode

This crate is meant for use with [sqlx](https://github.com/launchbadge/sqlx) and allows you to query JSON or JSONB fields from PostgreSQL without needing to wrap the types in a `sqlx::types::Json<>` wrapper type.

```rust
use serde::{Deserialize, Serialize};
use sqlx_transparent_json_decode::sqlx_json_decode;

#[derive(Serialize, Deserialize)]
pub struct SomeJsonField {
    // Whatever fields match the JSON structure
    pub name: String,
    pub some_param: Option<String>,
    pub count: i32,
}

sqlx_json_decode!(SomeJsonField);

#[derive(sqlx::FromRow)]
pub struct QueryResult {
    pub id: i32,
    pub name: String,
    pub params: SomeJsonField,
}
```

Normally, you would need to use `Json<SomeJsonField>` as the type for `params` in the above example. This crate allows you to use `SomeJsonField` directly.

```rust
let result = sqlx::query_as::<_, QueryResult>(
    r##"SELECT id,
        name,
        params as "params: SomeJsonField"
      FROM some_table"##,
).fetch_one(&pool).await?;
```
