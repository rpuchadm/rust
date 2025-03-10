use rocket::FromForm;
use rocket::form::Form;
use rocket::http::Status;
use chrono::prelude::*;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{ get, launch, post, routes, State}; // delete, put
use sqlx::{Decode, FromRow};

struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Decode, Debug)]
struct Session {
    id: i32,
    //#[serde(skip_serializing)]
    //code: String,
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

#[derive(Deserialize, Serialize, FromForm)]
struct AccessTokenRequest {
    //grant_type: String,
    client_id: String,
    //client_secret: String,
    //redirect_uri: String,
    code: String,
}

// a post accessToken se accede 1 sola vez con el code para obtener el token
#[post("/accessToken", data = "<request>")]
async fn access_token(state: &State<AppState>, request: Form<AccessTokenRequest>) -> Result<Json<Session>, Status> {
    // Aquí puedes procesar los datos del formulario
    //let grant_type = &request.grant_type;
    let client_id = &request.client_id;
    //let client_secret = &request.client_secret;
    //let redirect_uri = &request.redirect_uri;
    let code = &request.code;

    print!("access_token client_id: {}, code: {}", client_id, code);

    let pool = state.pool.clone();

    // Obtiene la sesión por el código de autorización y el id del cliente
    // si no encuentra la sesión devuelve un error 500
    let session = postgres_get_session_by_code_client_id(&pool, code, client_id)
        .await
        .map_err(|err| {
            eprintln!("Error getting session by code={} and client_id={} : {:?}", code, client_id, err);
            Status::Forbidden
        })?;

    // Log para inspeccionar la sesión
    println!("access_token Session: {:?}", session);

    // Verifica que el código de autorización sea válido
    if session.token.is_empty() {
        eprintln!("Error empty token for code={} and client_id={}", code, client_id);
        return Err(Status::Forbidden);
    }

    // Actualiza el código de autorización a nulo
    postgres_update_session_set_code_null_by_id(&pool, session.id)
        .await
        .or_else(|err| {
            eprintln!("Error updating session code to null by id={} : {:?}", session.id, err);
            Err(err)
        })
        .unwrap();

    Ok(Json(session.clone()))
}

// a get profile se accede de manera reiterada con el token para obtener la sesión
#[get("/profile")]
async fn profile(state: &State<AppState>, token: BearerToken) -> Result<Json<Session>, Status> {
    let pool = state.pool.clone();
    let session = postgres_get_session_by_token(&pool, &token.0)
        .await
        .map_err(|err| {
            eprintln!("Error getting session by token={} : {:?}", token.0, err);
            Status::Forbidden
        })?;

    // Log para inspeccionar la sesión
    println!("Session: {:?}", session);

    Ok(Json(session.clone()))
}

#[launch]
async fn rocket() -> _ {

    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPool::connect(POSTGRES_SERVER)
    .await.or_else(|err| {
        eprintln!("Error connecting to the database: {:?}", err);
        Err(err)
    })
    .unwrap();

    // intenta ejecutar DROP TABLE IF EXISTS sessions
    sqlx::query(
        r#"
        DROP TABLE IF EXISTS sessions
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    // intenta ejecutar CREATE TABLE IF NOT EXISTS sessions
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id SERIAL PRIMARY KEY,
            code TEXT,
            client_id TEXT NOT NULL,
            token TEXT NOT NULL UNIQUE,
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

    // crea indice para code y client_id cuando code no es nulo
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_sessions_code_client_id
        ON sessions (code, client_id)
        WHERE code IS NOT NULL
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    let expires_at: NaiveDateTime = (Utc::now() + chrono::Duration::days(1)).naive_utc();
    // intenta ejecutar INSERT INTO sessions (token, user_id, expires_at, attributes) VALUES ($1, $2, $3, $4)
    sqlx::query(
        r#"
        INSERT INTO sessions (
            code, client_id, token, 
            user_id, expires_at, attributes
        ) VALUES (
            $1, $2, $3,
            $4, $5, $6
        )
        "#
    )
    .bind("mycode")
    .bind("myclientid")
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
        .mount("/", routes![profile, access_token])
}

// constante con el servidor de postgres
const POSTGRES_SERVER: &str = "postgresql://myuser:mypassword@localhost:5432/mydatabase";

async fn postgres_get_session_by_token( pool: &sqlx::Pool<sqlx::Postgres>, token: &str) -> Result<Session, sqlx::Error> {
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

async fn postgres_get_session_by_code_client_id( pool: &sqlx::Pool<sqlx::Postgres>, code: &str, client_id: &str) -> Result<Session, sqlx::Error> {
    let session = sqlx::query_as::<_, Session>(
        r#"
        SELECT id, token, user_id, created_at, expires_at, attributes
        FROM sessions
        WHERE code = $1 and client_id = $2
        "#
    )
    .bind(code)
    .bind(client_id)
    .fetch_one(pool)
    .await?;

    Ok(session)
}

async fn postgres_update_session_set_code_null_by_id( pool: &sqlx::Pool<sqlx::Postgres>, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET code = NULL
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}