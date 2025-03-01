fn main() {
    // Operaciones con enteros (i32)
    let a: i32 = 10;
    let b: i32 = 4;
    println!("Operaciones con enteros (i32):");
    println!("a = {}, b = {}", a, b);
    println!("Suma: a + b = {}", a + b);
    println!("Resta: a - b = {}", a - b);
    println!("Multiplicación: a * b = {}", a * b);
    println!("División entera: a / b = {}", a / b); // Resultado: 2 (división entera)
    println!("Módulo: a % b = {}", a % b); // Resto de la división
    println!();

    // Operaciones con flotantes (f64)
    let x: f64 = 10.5;
    let y: f64 = 2.5;
    println!("Operaciones con flotantes (f64):");
    println!("x = {}, y = {}", x, y);
    println!("Suma: x + y = {}", x + y);
    println!("Resta: x - y = {}", x - y);
    println!("Multiplicación: x * y = {}", x * y);
    println!("División: x / y = {}", x / y);
    println!();

    // Operaciones con booleanos (bool)
    let verdadero: bool = true;
    let falso: bool = false;
    println!("Operaciones con booleanos (bool):");
    println!("verdadero = {}, falso = {}", verdadero, falso);
    println!("AND: verdadero && falso = {}", verdadero && falso);
    println!("OR: verdadero || falso = {}", verdadero || falso);
    println!("NOT: !verdadero = {}", !verdadero);
    println!();

    // Operaciones con caracteres (char)
    let letra1: char = 'A';
    let letra2: char = 'B';
    println!("Operaciones con caracteres (char):");
    println!("letra1 = {}, letra2 = {}", letra1, letra2);
    println!("Concatenación (usando strings): {} + {} = {}", letra1, letra2, format!("{}{}", letra1, letra2));
    println!();

    // Operaciones con strings (String y &str)
    let saludo: String = String::from("Hola");
    let nombre: &str = "Rust";
    println!("Operaciones con strings:");
    println!("saludo = {}, nombre = {}", saludo, nombre);
    // Usar format! para evitar mover saludo
    let concatenacion = format!("{} {}", saludo, nombre);
    println!("Concatenación: {}", concatenacion);

    // Ahora puedes usar saludo sin problemas
    println!("Longitud del saludo: {}", saludo.len());
    println!("¿El saludo contiene 'la'? {}", saludo.contains("la"));
    println!();

    // Operaciones con tuplas
    let tupla: (i32, f64, char) = (42, 3.14, 'R');
    println!("Operaciones con tuplas:");
    println!("tupla = {:?}", tupla);
    println!("Primer elemento: {}", tupla.0);
    println!("Segundo elemento: {}", tupla.1);
    println!("Tercer elemento: {}", tupla.2);
    println!();

    // Operaciones con arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Operaciones con arrays:");
    println!("array = {:?}", array);
    println!("Primer elemento: {}", array[0]);
    println!("Segundo elemento: {}", array[1]);
    println!("Longitud del array: {}", array.len());
    println!();

    // ----------------------------------------
    // Operaciones con cadenas de texto (String)

    // String: Propietario, mutable, almacenado en el heap
    let mut saludo: String = String::from("Hola");
    saludo.push_str(", mundo!"); // Modificar el String
    println!("{}", saludo); // Imprime: "Hola, mundo!"

    // &str: Referencia, inmutable, puede estar en el heap o en memoria estática
    let nombre: &str = "Rust"; // String literal (memoria estática)
    println!("{}", nombre); // Imprime: "Rust"

    // Convertir String a &str
    let saludo_slice: &str = &saludo;
    println!("{}", saludo_slice); // Imprime: "Hola, mundo!"

    // Concatenar String y &str
    let mensaje_completo = saludo + " " + nombre;
    println!("{}", mensaje_completo); // Imprime: "Hola, mundo! Rust"

}