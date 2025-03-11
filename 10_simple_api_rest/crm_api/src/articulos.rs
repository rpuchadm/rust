#[derive(Serialize, Clone)]
struct Articulo {
    id: i32,
    nombre: String,
    descripcion: Option<String>,
    precio: f64,
    stock: i32,
    fecha_creacion: chrono::NaiveDateTime,
}

#[get("/articulos")]
async fn articulos(state: &rocket::State<AppState>) -> Result<Vec<Json<Cliente>>, Status> {
    let pool = state.pool.clone();
    let articulos = postgres_get_articulos(&pool).await.map_err(|e| {
        eprintln!("Error getting articles: {:?}", e);
        Status::InternalServerError
    })?;

    Ok(articulos.into_iter().map(Json).collect())
}

async fn postgres_get_articulos(pool: &sqlx::Pool<sqlx::Postgres>) -> Vec<Articulo> {
    let articulos = sqlx::query_as::<_, Articulo>(
        "
        SELECT
            id, nombre, descripcion, precio, stock, fecha_creacion
        FROM articulos",
    )
    .fetch_all(pool)
    .await?;
}
