use chrono::prelude::*;
use redis::AsyncCommands;
use rocket::FromForm;
use rocket::form::Form;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{State, delete, get, launch, post, routes}; // put
use sqlx::{Decode, FromRow};

struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
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

    postgresini(pool.clone()).await;

    rocket::build()
        .manage(AppState { pool })
        .mount("/", routes![articulos, profile])
}

#[derive(Serialize, Clone, FromRow)]
struct Articulo {
    id: i32,
    nombre: String,
    descripcion: Option<String>,
    precio: i32,
    stock: i32,
    fecha_creacion: chrono::NaiveDateTime,
}

#[get("/articulos")]
async fn articulos(state: &rocket::State<AppState>) -> Result<Json<Vec<Articulo>>, Status> {
    let pool = state.pool.clone();
    let articulos = postgres_get_articulos(&pool).await.map_err(|e| {
        eprintln!("Error getting articles: {:?}", e);
        Status::InternalServerError
    })?;

    Ok(Json(articulos))
}

async fn postgres_get_articulos(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<Articulo>, sqlx::Error> {
    let articulos = sqlx::query_as::<_, Articulo>(
        "
        SELECT
            id, nombre, descripcion, precio, stock, fecha_creacion
        FROM articulos",
    )
    .fetch_all(pool)
    .await?;

    Ok(articulos)
}

#[derive(Serialize, Clone, FromRow)]
struct Cliente {
    id: i32,
    user_id: i32,
    nombre: String,
    email: String,
    telefono: Option<String>,
    direccion: Option<String>,
    fecha_registro: chrono::NaiveDateTime,
}

#[get("/profile/<user_id>")]
async fn profile(state: &State<AppState>, user_id: i32) -> Result<Json<Cliente>, Status> {
    let pool = state.pool.clone();

    let cliente = postgres_get_cliente_by_id(&pool, user_id)
        .await
        .map_err(|e| {
            eprintln!("Error getting client: {:?}", e);
            Status::InternalServerError
        })?;

    Ok(Json(cliente))
}

async fn postgres_get_cliente_by_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Cliente, sqlx::Error> {
    let cliente: Cliente = sqlx::query_as::<_, Cliente>(
        "SELECT
            id, user_id, nombre, email, telefono, direccion, fecha_registro
        FROM clientes
        WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(cliente)
}

// constante con el servidor de postgres
const POSTGRES_SERVER: &str = "postgresql://myuser:mypassword@localhost:5432/mydatabase";

async fn postgresini(pool: sqlx::Pool<sqlx::Postgres>) {
    sqlx::query(
        r#"        
        DROP TABLE IF EXISTS pedidos_detalles;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        DROP TABLE IF EXISTS pedidos;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        DROP TABLE IF EXISTS clientes;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        CREATE TABLE IF NOT EXISTS clientes (
            id SERIAL PRIMARY KEY,              -- Identificador único del cliente
            user_id INT NOT NULL UNIQUE,        -- Identificador único del usuario, integra con auth
            nombre VARCHAR(100) NOT NULL,       -- Nombre del cliente
            email VARCHAR(100) UNIQUE,          -- Email del cliente (único)
            telefono VARCHAR(20),               -- Teléfono del cliente
            direccion TEXT,                     -- Dirección del cliente
            fecha_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Fecha de registro
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        DROP TABLE IF EXISTS articulos;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        CREATE TABLE IF NOT EXISTS articulos (
            id SERIAL PRIMARY KEY,              -- Identificador único del artículo
            nombre VARCHAR(100) NOT NULL,       -- Nombre del artículo
            descripcion TEXT,                   -- Descripción del artículo
            precio INT NOT NULL,                -- Precio del artículo en centimos
            stock INT NOT NULL DEFAULT 0,       -- Cantidad en stock
            fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Fecha de creación
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        CREATE TABLE IF NOT EXISTS pedidos (
            id SERIAL PRIMARY KEY,              -- Identificador único del pedido
            cliente_id INT NOT NULL,             -- ID del cliente que realiza el pedido
            fecha_pedido TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Fecha del pedido
            estado VARCHAR(50) NOT NULL DEFAULT 'Pendiente', -- Estado del pedido (Pendiente, Enviado, Entregado, etc.)
            total INT NOT NULL,      -- Total del pedido en centimos
            FOREIGN KEY (cliente_id) REFERENCES clientes(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        CREATE TABLE IF NOT EXISTS pedidos_detalles (
            id SERIAL PRIMARY KEY,              -- Identificador único del detalle
            pedido_id INT NOT NULL,             -- ID del pedido
            articulo_id INT NOT NULL,           -- ID del artículo
            cantidad INT NOT NULL,              -- Cantidad del artículo en el pedido
            precio_unitario INT NOT NULL, -- Precio unitario del artículo en el momento del pedido
            subtotal INT NOT NULL,   -- Subtotal (cantidad * precio_unitario)
            FOREIGN KEY (pedido_id) REFERENCES pedidos(id) ON DELETE CASCADE,
            FOREIGN KEY (articulo_id) REFERENCES articulos(id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        INSERT INTO clientes (user_id,nombre, email, telefono, direccion)
        VALUES (21,'Juan Pérez', 'juan@example.com', '123456789', 'Calle Falsa 123');
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        INSERT INTO articulos (nombre, descripcion, precio, stock)
        VALUES ('Laptop', 'Laptop de 15 pulgadas', 120000, 10);
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        INSERT INTO pedidos (cliente_id, total)
        VALUES (1, 240000); -- Suponiendo que el cliente con ID 1 compra 2 laptops
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"        
        INSERT INTO pedidos_detalles (pedido_id, articulo_id, cantidad, precio_unitario, subtotal)
        VALUES (1, 1, 2, 120000, 240000); -- 2 laptops a 1200.00 cada una
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
}
