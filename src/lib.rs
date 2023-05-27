pub const JSON_OID: sqlx::postgres::types::Oid = sqlx::postgres::types::Oid(114);
pub const JSONB_OID: sqlx::postgres::types::Oid = sqlx::postgres::types::Oid(3802);

#[macro_export]
macro_rules! sqlx_json_decode {
    ($type:ty) => {
        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $type {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                use sqlx::ValueRef;
                let is_jsonb = value.type_info().as_ref()
                    == &sqlx::postgres::PgTypeInfo::with_oid($crate::JSONB_OID);
                let mut buf = <&[u8] as sqlx::Decode<sqlx::Postgres>>::decode(value)?;

                if is_jsonb {
                    assert_eq!(
                        buf[0], 1,
                        "unsupported JSONB format version {}; please open an issue",
                        buf[0]
                    );

                    buf = &buf[1..];
                }
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
    };
}
