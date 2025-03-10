use chrono::prelude::*;
use rocket::FromForm;
use rocket::form::Form;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{State, get, launch, post, routes}; // delete, put
use sqlx::{Decode, FromRow};

// declara string con token supersecreto que permite crear sesiones nuevas y desactivar viejas
const SUPER_SECRET: &str = "mysupersecret"; // en un futuro se sacará de ENV

struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(Serialize, Deserialize, Clone, FromRow, Decode, Debug)]
struct Session {
    id: i32,
    client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
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
async fn access_token(
    state: &State<AppState>,
    request: Form<AccessTokenRequest>,
) -> Result<Json<Session>, Status> {
    // procesar los datos del formulario
    //let grant_type = &request.grant_type;
    let client_id = &request.client_id;
    //let client_secret = &request.client_secret;
    //let redirect_uri = &request.redirect_uri;
    let code = &request.code;

    print!("access_token client_id: {}, code: {}", client_id, code);

    let pool = state.pool.clone();

    // Obtiene la sesión por el código de autorización y el id del cliente
    let mut session = postgres_get_session_by_code_client_id(&pool, code, client_id)
        .await
        .map_err(|err| {
            eprintln!(
                "Error getting session by code={} and client_id={} : {:?}",
                code, client_id, err
            );
            Status::Forbidden
        })?;

    // Log para inspeccionar la sesión
    println!("access_token Session: {:?}", session);

    let token = random_token(128);

    // Actualiza el código de autorización a nulo
    postgres_update_session_set_token_codenull_by_id(&pool, session.id, &token)
        .await
        .or_else(|err| {
            eprintln!(
                "Error updating session token and code to null by id={} : {:?}",
                session.id, err
            );
            Err(Status::InternalServerError)
        })
        .unwrap();

    session.token = Some(token);
    session.code = None;

    Ok(Json(session.clone()))
}

// a get profile se accede de manera reiterada con el token para obtener la sesión
#[get("/profile")]
async fn profile(state: &State<AppState>, token: BearerToken) -> Result<Json<Session>, Status> {
    let pool = state.pool.clone();
    let mut session = postgres_get_session_by_codenull_token(&pool, &token.0)
        .await
        .map_err(|err| {
            eprintln!(
                "Error getting session by code null and token={} : {:?}",
                token.0, err
            );
            Status::Forbidden
        })?;

    // Log para inspeccionar la sesión
    //println!("Session: {:?}", session);

    session.token = None;

    Ok(Json(session.clone()))
}

#[derive(Deserialize)]
struct SessionRequest {
    client_id: String,
    user_id: i32,
    expires_in_min: i64,
    attributes: serde_json::Value,
}

#[post("/session", data = "<session_request>")]
async fn new_session(
    state: &State<AppState>,
    session_request: Json<SessionRequest>,
    token: BearerToken,
) -> Result<Json<Session>, Status> {
    if token.0 != SUPER_SECRET {
        eprintln!("Error invalid super secret token");
        return Err(Status::Unauthorized);
    }

    let code = random_token(32);

    let pool = state.pool.clone();

    let expires_at: NaiveDateTime =
        (Utc::now() + chrono::Duration::minutes(session_request.expires_in_min)).naive_utc();

    postgres_insert_session(
        &pool,
        &code,
        &session_request.client_id,
        session_request.user_id,
        expires_at,
        session_request.attributes.clone(),
    )
    .await
    .or_else(|err| {
        eprintln!("Error inserting session : {:?}", err);
        Err(Status::InternalServerError)
    })?;

    let code_some: Option<String> = Some(code);
    let token_none: Option<String> = None;

    let session = Session {
        id: 0,
        client_id: session_request.client_id.clone(),
        code: code_some,
        token: token_none,
        user_id: session_request.user_id,
        created_at: chrono::Utc::now().naive_utc(),
        expires_at,
        attributes: session_request.attributes.clone(),
    };

    Ok(Json(session))
}

#[launch]
async fn rocket() -> _ {
    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPool::connect(POSTGRES_SERVER)
        .await
        .or_else(|err| {
            eprintln!("Error connecting to the database: {:?}", err);
            Err(err)
        })
        .unwrap();

    ini_postgres(pool.clone()).await;

    rocket::build()
        .manage(AppState { pool })
        .mount("/", routes![access_token, new_session, profile])
}

async fn ini_postgres(pool: sqlx::Pool<sqlx::Postgres>) {
    sqlx::query(
        r#"        
        DROP INDEX IF EXISTS idx_sessions_code_client_id;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        DROP INDEX IF EXISTS idx_sessions_token;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        DROP TABLE IF EXISTS sessions;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id SERIAL PRIMARY KEY,
            client_id TEXT NOT NULL,
            code TEXT,
            token TEXT,
            user_id INTEGER NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            expires_at TIMESTAMP NOT NULL,
            attributes JSONB NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    // Crea un índice para que code y client_id sean únicos cuando code no sea null
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_sessions_code_client_id
        ON sessions (code, client_id)
        WHERE code IS NOT NULL
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    // Crea un índice para que token sea único cuando no sea null
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_sessions_token
        ON sessions (token)
        WHERE token IS NOT NULL
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
}
// constante con el servidor de postgres
const POSTGRES_SERVER: &str = "postgresql://myuser:mypassword@localhost:5432/mydatabase";

async fn postgres_get_session_by_codenull_token(
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

async fn postgres_get_session_by_code_client_id(
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

async fn postgres_update_session_set_token_codenull_by_id(
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

async fn postgres_insert_session(
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

// funcion para generar un token aleatorio de n caracteres
fn random_token(n: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{Rng, thread_rng};

    thread_rng()
        .sample_iter(&Alphanumeric) // Genera una secuencia de caracteres alfanuméricos
        .take(n) // Toma `n` caracteres
        .map(char::from) // Convierte cada u8 a char
        .collect() // Recolecta en un String
}
