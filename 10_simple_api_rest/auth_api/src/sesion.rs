use rocket::serde::{Deserialize, Serialize};
use sqlx::{Decode, FromRow};

#[derive(Serialize, Deserialize, Clone, FromRow, Decode, Debug)]
pub struct Session {
    pub id: i32,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub attributes: serde_json::Value,
}

pub async fn postgres_insert_session(
    pool: &sqlx::Pool<sqlx::Postgres>,
    code: &str,
    client_id: &str,
    user_id: i32,
    expires_at: chrono::NaiveDateTime,
    attributes: serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sessions (
            code, client_id, user_id,
            expires_at, attributes
        ) VALUES (
            $1, $2, $3,
            $4, $5
        )
        "#,
    )
    .bind(code)
    .bind(client_id)
    .bind(user_id)
    .bind(expires_at)
    .bind(attributes)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn postgres_get_session_by_codenull_token(
    pool: &sqlx::Pool<sqlx::Postgres>,
    token: &str,
) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as::<_, Session>(
        r#"
        SELECT
            id, client_id, code, token, user_id, created_at, expires_at, attributes
        FROM sessions
        WHERE code IS NULL and token = $1
        and expires_at > now()
        "#,
    )
    .bind(token)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn postgres_get_session_by_code_client_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    code: &str,
    client_id: &str,
) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as::<_, Session>(
        r#"
        SELECT
            id, client_id, code, token, user_id, created_at, expires_at, attributes
        FROM sessions
        WHERE code = $1 and client_id = $2
        and expires_at > now()
        "#,
    )
    .bind(code)
    .bind(client_id)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

pub async fn postgres_update_session_set_token_codenull_by_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: i32,
    token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET token = $1, code = NULL
        WHERE id = $2
        "#,
    )
    .bind(token)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn postgres_update_session_token_null_closed_at_by_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET token = NULL, closed_at = now()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}
