use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// Función que lee un archivo y devuelve su contenido como un String
fn leer_archivo(ruta: &str) -> Result<String, io::Error> {
    let mut archivo = File::open(ruta)?; // Operador ? para propagar errores
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?; // Operador ? para propagar errores
    Ok(contenido)
}

// Función que convierte un String en un número entero
fn convertir_a_numero(texto: &str) -> Result<i32, ParseIntError> {
    let numero = texto.trim().parse::<i32>()?; // Operador ? para propagar errores
    Ok(numero)
}

// Función que busca un número en una lista
fn buscar_numero(lista: &[i32], objetivo: i32) -> Option<usize> {
    lista.iter().position(|&x| x == objetivo)
}

fn main() {
    // Ejemplo 1: Manejo de errores con Result y el operador ?
    match leer_archivo("datos.txt") {
        Ok(contenido) => {
            println!("Contenido del archivo:\n{}", contenido);

            // Ejemplo 2: Convertir el contenido a un número
            match convertir_a_numero(&contenido) {
                Ok(numero) => {
                    println!("Número leído: {}", numero);

                    // Ejemplo 3: Buscar el número en una lista
                    let lista = vec![10, 20, 30, 40, 50];
                    match buscar_numero(&lista, numero) {
                        Some(indice) => println!("El número {} está en la posición {}", numero, indice),
                        None => println!("El número {} no está en la lista", numero),
                    }
                }
                Err(e) => println!("Error al convertir el contenido a número: {}", e),
            }
        }
        Err(e) => println!("Error al leer el archivo: {}", e),
    }
}