#[derive(Serialize, Clone)]
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

async fn postgres_get_cliente_by_id(pool: &sqlx::Pool<sqlx::Postgres>, user_id: i32) -> User {
    let cliente = sqlx::query_as::<_, Cliente>(
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
