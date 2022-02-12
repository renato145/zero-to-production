use super::IdempotencyKey;
use actix_web::{http::StatusCode, HttpResponse};
use sqlx::{postgres::PgHasArrayType, PgPool};
use uuid::Uuid;

#[derive(Debug)]
struct HttpResponseRecord {
    status_code: i16,
    headers: Vec<HeaderPairRecord>,
    body: Vec<u8>,
}

impl sqlx::Type<sqlx::Postgres> for HttpResponseRecord {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("http_response")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for HttpResponseRecord {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let status_code = decoder.try_decode::<i16>()?;
        let headers = decoder.try_decode::<Vec<HeaderPairRecord>>()?;
        let body = decoder.try_decode::<Vec<u8>>()?;
        Ok(Self {
            status_code,
            headers,
            body,
        })
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "header_pair")]
struct HeaderPairRecord {
    name: String,
    value: Vec<u8>,
}

impl PgHasArrayType for HeaderPairRecord {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        // Array type name is the name of the element type prefixed with `_`
        sqlx::postgres::PgTypeInfo::with_name("_header_pair")
    }
}

pub async fn get_saved_response(
    pool: &PgPool,
    idempotency_key: &IdempotencyKey,
    user_id: Uuid,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    let saved_response = sqlx::query!(
        r#"
        SELECT http_response as "http_response: HttpResponseRecord"
        FROM idempotency
        WHERE 
          user_id = $1 AND
          idempotency_key = $2
        "#,
        user_id,
        idempotency_key.as_ref()
    )
    .fetch_optional(pool)
    .await?;
    if let Some(r) = saved_response {
        let status_code = StatusCode::from_u16(r.http_response.status_code.try_into()?)?;
        let mut response = HttpResponse::build(status_code);
        for HeaderPairRecord { name, value } in r.http_response.headers {
            response.append_header((name, value));
        }
        Ok(Some(response.body(r.http_response.body)))
    } else {
        Ok(None)
    }
}

pub async fn save_response(
    _pool: &PgPool,
    _idempotency_key: &IdempotencyKey,
    _user_id: Uuid,
) -> Result<Option<HttpResponse>, anyhow::Error> {
    todo!()
}