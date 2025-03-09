use rocket::http::Status;
use chrono::prelude::*;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{delete, get, launch, post, put, routes, State};
use sqlx::{Decode, FromRow};

struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Decode)]
struct Session {
    id: i32,
    token: String,
    user_id: i32,
    created_at: chrono::NaiveDateTime,
    expires_at: chrono::NaiveDateTime,
    attributes: serde_json::Value,
}

struct BearerToken(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BearerToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = auth_header[7..].to_string();
                return Outcome::Success(BearerToken(token));
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[get("/profile")]
async fn profile(state: &State<AppState>, token: BearerToken) -> Option<Json<Session>> {
    let pool = state.pool.clone();
    let session = postgres_get_session(&pool, &token.0)
        .await
        .ok()?;
    Some(Json(session.clone()))
}

#[launch]
async fn rocket() -> _ {

    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPool::connect(POSTGRES_SERVER)
    .await.or_else(|err| {
        eprintln!("Error connecting to the database: {:?}", err);
        Err(err)
    })
    .unwrap();

    // intenta ejecutar CREATE TABLE IF NOT EXISTS sessions
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id SERIAL PRIMARY KEY,
            token TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            expires_at TIMESTAMP NOT NULL,
            attributes JSONB NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let expires_at: NaiveDateTime = (Utc::now() + chrono::Duration::days(1)).naive_utc();
    // intenta ejecutar INSERT INTO sessions (token, user_id, expires_at, attributes) VALUES ($1, $2, $3, $4)
    sqlx::query(
        r#"
        INSERT INTO sessions (token, user_id, expires_at, attributes) VALUES ($1, $2, $3, $4)
        "#
    )
    .bind("mytoken")
    .bind(1)
    .bind( expires_at)
    .bind(serde_json::json!({"key": "value"}))
    .execute(&pool)
    .await
    .unwrap();

    rocket::build()
        .manage(AppState {
            pool,
        })
        .mount("/", routes![profile])
}

// constante con el servidor de postgres
const POSTGRES_SERVER: &str = "postgresql://myuser:mypassword@localhost:5432/mydatabase";

async fn postgres_get_session( pool: &sqlx::Pool<sqlx::Postgres>, token: &str) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as::<_, Session>(
        r#"
        SELECT id, token, user_id, created_at, expires_at, attributes
        FROM sessions
        WHERE token = $1
        "#
    )
    .bind(token)
    .fetch_one(pool)
    .await?;

    Ok(session)
}