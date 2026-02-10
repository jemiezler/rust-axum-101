use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalizedPersonName  {
    pub prefix: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
}

impl sqlx::Type<sqlx::Postgres> for LocalizedPersonName  {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("JSONB")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for LocalizedPersonName  {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let wrapper: sqlx::types::Json<Self> =
            <sqlx::types::Json<Self> as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(wrapper.0)
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for LocalizedPersonName  {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        <sqlx::types::Json<&Self> as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(
            &sqlx::types::Json(self),
            buf,
        )
    }
}
