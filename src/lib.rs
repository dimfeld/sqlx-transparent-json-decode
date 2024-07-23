//! This crate is meant for use with [sqlx](https://github.com/launchbadge/sqlx) and allows you to query JSON
//! or JSONB fields from PostgreSQL without needing to wrap the types in a `sqlx::types::Json<>` wrapper type.
//!

use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

#[cfg(test)]
mod test;

#[doc(hidden)]
/// This must be exported for the macro to work, but you won't need to use it.
pub const JSON_OID: sqlx::postgres::types::Oid = sqlx::postgres::types::Oid(114);
#[doc(hidden)]
/// This must be exported for the macro to work, but you won't need to use it.
pub const JSON_ARRAY_OID: sqlx::postgres::types::Oid = sqlx::postgres::types::Oid(199);
#[doc(hidden)]
/// This must be exported for the macro to work, but you won't need to use it.
pub const JSONB_OID: sqlx::postgres::types::Oid = sqlx::postgres::types::Oid(3802);
#[doc(hidden)]
/// This must be exported for the macro to work, but you won't need to use it.
pub const JSONB_ARRAY_OID: sqlx::postgres::types::Oid = sqlx::postgres::types::Oid(3807);

/// Generate a Decode implementation for a type that can read it from a PostgreSQL JSON/JSONB field.
///
/// ```ignore
/// use serde::{Deserialize, Serialize};
/// use sqlx_transparent_json_decode::sqlx_json_decode;
///
/// #[derive(Serialize, Deserialize)]
/// pub struct SomeJsonField {
///     // Whatever fields match the JSON structure
///     pub name: String,
///     pub some_param: Option<String>,
///     pub count: i32,
/// }
///
/// sqlx_json_decode!(SomeJsonField);
///
/// #[derive(sqlx::FromRow)]
/// pub struct QueryResult {
///     pub id: i32,
///     pub name: String,
///     pub params: SomeJsonField,
/// }
/// ```
///
/// Normally, you would need to use `Json<SomeJsonField>` as the type for `params` in the above example. This macro allows you to use `SomeJsonField` directly.
///
/// ```ignore
/// let result = sqlx::query_as!(
///     QueryResult,
///     r##"SELECT id,
///         name,
///         params as "params: SomeJsonField"
///       FROM some_table"##,
/// ).fetch_one(&pool).await?;
/// ```
#[macro_export]
macro_rules! sqlx_json_decode {
    ($type:ty) => {
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $type {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let buf = $crate::decode_json(value)?;
                serde_json::from_slice(buf).map_err(Into::into)
            }
        }

        impl sqlx::Type<sqlx::Postgres> for $type {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                sqlx::postgres::PgTypeInfo::with_oid($crate::JSONB_OID)
            }

            fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                *ty == sqlx::postgres::PgTypeInfo::with_oid($crate::JSONB_OID)
                    || *ty == sqlx::postgres::PgTypeInfo::with_oid($crate::JSON_OID)
            }
        }

        impl sqlx::postgres::PgHasArrayType for $type {
            fn array_type_info() -> sqlx::postgres::PgTypeInfo {
                sqlx::postgres::PgTypeInfo::with_oid($crate::JSONB_ARRAY_OID)
            }

            fn array_compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                *ty == sqlx::postgres::PgTypeInfo::with_oid($crate::JSONB_ARRAY_OID)
                    || *ty == sqlx::postgres::PgTypeInfo::with_oid($crate::JSON_ARRAY_OID)
            }
        }
    };
}

/// A wrapper around Box<serde_json::value::RawValue> which can be decoded directly from Postgres.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoxedRawValue(Box<serde_json::value::RawValue>);

#[cfg(feature = "boxed_raw_value_eq")]
impl PartialEq for BoxedRawValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

#[cfg(feature = "boxed_raw_value_eq")]
impl Eq for BoxedRawValue {}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for BoxedRawValue {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        serde_json::value::RawValue::schema_name()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        serde_json::value::RawValue::schema_id()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        serde_json::value::RawValue::json_schema(gen)
    }
}

impl std::ops::Deref for BoxedRawValue {
    type Target = serde_json::value::RawValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BoxedRawValue> for Box<serde_json::value::RawValue> {
    fn from(raw_value: BoxedRawValue) -> Self {
        raw_value.0
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for BoxedRawValue {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let buf = decode_json(value)?;
        let string = std::str::from_utf8(buf)?;
        let raw_value = serde_json::value::RawValue::from_string(string.to_owned())?;
        Ok(BoxedRawValue(raw_value))
    }
}

impl PgHasArrayType for BoxedRawValue {
    fn array_type_info() -> PgTypeInfo {
        serde_json::value::RawValue::array_type_info()
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        serde_json::value::RawValue::array_compatible(ty)
    }
}

impl<'r> sqlx::Encode<'r, sqlx::Postgres> for BoxedRawValue {
    fn encode_by_ref(
        &self,
        out: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let j = sqlx::types::Json(&self.0);
        <sqlx::types::Json<&Box<sqlx::types::JsonRawValue>> as sqlx::Encode<'_, sqlx::Postgres>>::encode_by_ref(
            &j, out,
        )
    }
}

impl sqlx::Type<sqlx::Postgres> for BoxedRawValue {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_oid(JSONB_OID)
    }

    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
        *ty == sqlx::postgres::PgTypeInfo::with_oid(JSONB_OID)
            || *ty == sqlx::postgres::PgTypeInfo::with_oid(JSON_OID)
    }
}

/// Extract a byte slice from a Postgres JSON or JSONB value. You shouldn't need to use this directly.
pub fn decode_json(
    value: <sqlx::Postgres as sqlx::Database>::ValueRef<'_>,
) -> Result<&'_ [u8], sqlx::error::BoxDynError> {
    use sqlx::ValueRef;
    let is_jsonb = value.type_info().as_ref() == &sqlx::postgres::PgTypeInfo::with_oid(JSONB_OID);
    let mut buf = <&[u8] as sqlx::Decode<sqlx::Postgres>>::decode(value)?;

    if is_jsonb {
        assert_eq!(
            buf[0], 1,
            "unsupported JSONB format version {}; please open an issue",
            buf[0]
        );

        buf = &buf[1..];
    }

    Ok(buf)
}
