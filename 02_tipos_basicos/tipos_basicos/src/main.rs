fn main() {
    // Tipos booleanos
    let es_verdadero: bool = true;
    let es_falso = false; // Rust infiere el tipo automáticamente
    println!("Tipos booleanos:");
    println!("es_verdadero: {}", es_verdadero);
    println!("es_falso: {}", es_falso);
    println!();

    // Tipos numéricos
    let entero: i32 = 42; // Entero de 32 bits con signo
    let flotante: f64 = 3.14; // Flotante de 64 bits
    let hexadecimal = 0x1A; // Entero en formato hexadecimal
    let binario = 0b1101; // Entero en formato binario
    println!("Tipos numéricos:");
    println!("entero: {}", entero);
    println!("flotante: {}", flotante);
    println!("hexadecimal: {}", hexadecimal);
    println!("binario: {}", binario);
    println!();

    // Caracteres
    let letra: char = 'R'; // Carácter Unicode
    let emoji = '😊'; // Emoji (también es un carácter en Rust)
    println!("Caracteres:");
    println!("letra: {}", letra);
    println!("emoji: {}", emoji);
    println!();

    // Strings
    let saludo = String::from("Hola, mundo!"); // String dinámico
    let nombre = "Rust"; // String literal (tipo &str)
    println!("Strings:");
    println!("saludo: {}", saludo);
    println!("nombre: {}", nombre);
    println!();

    // Tuplas
    let tupla: (i32, f64, char) = (42, 3.14, 'R'); // Tupla con múltiples tipos
    println!("Tuplas:");
    println!("tupla: {:?}", tupla);
    println!("Primer elemento de la tupla: {}", tupla.0);
    println!("Segundo elemento de la tupla: {}", tupla.1);
    println!("Tercer elemento de la tupla: {}", tupla.2);
    println!();

    // Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array de 5 elementos
    println!("Arrays:");
    println!("array: {:?}", array);
    println!("Primer elemento del array: {}", array[0]);
    println!("Segundo elemento del array: {}", array[1]);
    println!();

    // Operaciones básicas
    let suma = entero + 10;
    let division = flotante / 2.0;
    let concatenacion = format!("{} {}", saludo, nombre); // Concatenación de strings
    println!("Operaciones básicas:");
    println!("suma: {}", suma);
    println!("division: {}", division);
    println!("concatenacion: {}", concatenacion);
    println!();

    // Control de flujo con booleanos
    if es_verdadero {
        println!("El valor es verdadero.");
    } else {
        println!("El valor es falso.");
    }
}