// Importar el módulo modulos
mod modulos;

// Usar las funciones reexportadas
use modulos::{saludar, sumar};

fn main() {
    // Usar la función saludar
    saludar("Mundo");

    // Usar la función sumar
    let resultado = sumar(5, 3);
    println!("5 + 3 = {}", resultado);
}