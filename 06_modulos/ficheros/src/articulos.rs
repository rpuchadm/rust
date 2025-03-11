// articulos.rs

#[derive(Debug)]
pub struct Articulo {
    pub nombre: String,
    pub precio: f64,
}

impl Articulo {
    pub fn new(nombre: String, precio: f64) -> Self {
        Articulo { nombre, precio }
    }
}

pub fn mostrar_articulo(articulo: &Articulo) {
    println!(
        "Artículo: {}, Precio: {:.2}€",
        articulo.nombre, articulo.precio
    );
}
