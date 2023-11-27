mod sqlx_json_decode {
    use crate::sqlx_json_decode;
    use sqlx::FromRow;

    #[derive(Debug, serde::Deserialize)]
    struct JsonValue {
        s: String,
        i: i32,
    }

    sqlx_json_decode!(JsonValue);

    #[derive(Debug, FromRow)]
    struct Value {
        #[allow(dead_code)]
        id: i64,
        data: JsonValue,
    }

    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn query_as_fn(pool: sqlx::PgPool) {
        let value = sqlx::query_as::<_, Value>(r##"SELECT id, data FROM data WHERE id = 1"##)
            .fetch_one(&pool)
            .await
            .expect("fetching");

        assert_eq!(value.data.s, "abc");
        assert_eq!(value.data.i, 123);
    }

    // The macro and the function actually work very differently, so test them both.
    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn query_as_macro(pool: sqlx::PgPool) {
        let value = sqlx::query_as!(
            Value,
            r##"SELECT id, data as "data: JsonValue" FROM data WHERE id = 1"##
        )
        .fetch_one(&pool)
        .await
        .expect("fetching");

        assert_eq!(value.data.s, "abc");
        assert_eq!(value.data.i, 123);
    }

    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn query_macro(pool: sqlx::PgPool) {
        let value =
            sqlx::query!(r##"SELECT id, data as "data: JsonValue" FROM data WHERE id = 1"##)
                .fetch_one(&pool)
                .await
                .expect("fetching");

        assert_eq!(value.data.s, "abc");
        assert_eq!(value.data.i, 123);
    }
}

mod boxed_raw_value {
    use crate::BoxedRawValue;
    use serde_json::{json, value::RawValue};
    use sqlx::FromRow;

    #[derive(Debug, FromRow)]
    struct Value {
        #[allow(dead_code)]
        id: i64,
        data: BoxedRawValue,
    }

    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn query_as_fn(pool: sqlx::PgPool) {
        let value = sqlx::query_as::<_, Value>(r##"SELECT id, data FROM data WHERE id = 1"##)
            .fetch_one(&pool)
            .await
            .expect("fetching");

        let json_value: serde_json::Value = serde_json::from_str(value.data.get()).unwrap();
        assert_eq!(json_value, json!({"i": 123, "s": "abc"}));
    }

    // The macro and the function actually work very differently, so test them both.
    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn query_as_macro(pool: sqlx::PgPool) {
        let value = sqlx::query_as!(
            Value,
            r##"SELECT id, data as "data: BoxedRawValue" FROM data WHERE id = 1"##
        )
        .fetch_one(&pool)
        .await
        .expect("fetching");

        let json_value: serde_json::Value = serde_json::from_str(value.data.get()).unwrap();
        assert_eq!(json_value, json!({"i": 123, "s": "abc"}));
    }

    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn query_macro(pool: sqlx::PgPool) {
        let value =
            sqlx::query!(r##"SELECT id, data as "data: BoxedRawValue" FROM data WHERE id = 1"##)
                .fetch_one(&pool)
                .await
                .expect("fetching");

        let json_value: serde_json::Value = serde_json::from_str(value.data.get()).unwrap();
        assert_eq!(json_value, json!({"i": 123, "s": "abc"}));
    }

    #[sqlx::test(migrations = "./test_migrations")]
    pub async fn insert(pool: sqlx::PgPool) {
        let value = r##"{"i": 5, "s": "ppp"}"##;
        let boxed =
            BoxedRawValue(RawValue::from_string(value.to_string()).expect("Creating RawValue"));

        sqlx::query!(
            r##"INSERT INTO data (id, data) VALUES ($1, $2)"##,
            500,
            boxed as _
        )
        .execute(&pool)
        .await
        .unwrap();

        // Then read it back to make sure it worked properly
        let value =
            sqlx::query!(r##"SELECT id, data as "data: BoxedRawValue" FROM data WHERE id = 500"##)
                .fetch_one(&pool)
                .await
                .expect("fetching");

        let json_value: serde_json::Value = serde_json::from_str(value.data.get()).unwrap();
        assert_eq!(json_value, json!({"i": 5, "s": "ppp"}));
    }
}
