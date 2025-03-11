// main.rs

// Importar los módulos
mod articulos;
mod personas;

// Importar los structs y funciones
use articulos::{Articulo, mostrar_articulo};
use personas::{Persona, mostrar_persona};

fn main() {
    // Crear una persona
    let persona = Persona::new(String::from("Juan Pérez"), 30);
    mostrar_persona(&persona);

    // Crear un artículo
    let articulo = Articulo::new(String::from("Laptop"), 1200.50);
    mostrar_articulo(&articulo);
}
