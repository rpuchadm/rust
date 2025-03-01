fn main() {
    // Uso de `for` para iterar sobre un rango
    println!("Iterando con `for`:");
    for i in 0..5 { // Rango de 0 a 4 (no incluye el 5)
        println!("i = {}", i);
    }
    println!();

    // Uso de `for` con un array
    let frutas = ["manzana", "banana", "cereza"];
    println!("Iterando sobre un array con `for`:");
    for fruta in frutas.iter() {
        println!("Me gusta la {}", fruta);
    }
    println!();

    // Uso de `while` para un bucle condicional
    println!("Iterando con `while`:");
    let mut contador = 0;
    while contador < 3 {
        println!("contador = {}", contador);
        contador += 1; // Incrementa el contador
    }
    println!();

    // Uso de `loop` para un bucle infinito (con break)
    println!("Iterando con `loop`:");
    let mut intentos = 0;
    loop {
        println!("Intentando... ({})", intentos);
        intentos += 1;
        if intentos >= 3 {
            println!("¡Bucle infinito detenido!");
            break; // Sale del bucle
        }
    }
    println!();

    // Uso de `match` para coincidencia de patrones (similar a `case`)
    println!("Usando `match` para coincidencia de patrones:");
    let numero = 3;
    match numero {
        1 => println!("El número es uno."),
        2 | 3 => println!("El número es dos o tres."), // Coincidencia múltiple
        4..=10 => println!("El número está entre 4 y 10."), // Rango
        _ => println!("El número no coincide con ningún patrón."), // Caso por defecto
    }
    println!();

    // Uso de `if let` para coincidencia simple
    println!("Usando `if let` para coincidencia simple:");
    let opcion = Some(5);
    if let Some(valor) = opcion {
        println!("El valor es: {}", valor);
    } else {
        println!("No hay valor.");
    }
    println!();

    // Uso de `while let` para coincidencia en bucles
    println!("Usando `while let` para coincidencia en bucles:");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Sacando: {}", top);
    }
    println!();

    // Uso de `for` con enumeración
    println!("Usando `for` con enumeración:");
    for (indice, fruta) in frutas.iter().enumerate() {
        println!("Índice: {}, Fruta: {}", indice, fruta);
    }
    println!();
}