fn main() {
    // Ejemplo 1: Ownership y move
    let s1 = String::from("Hola");
    let s2 = s1; // s1 se mueve a s2 (ownership transferido)
    // println!("{}", s1); // Esto daría un error: value borrowed here after move
    println!("s2: {}", s2);

    // Ejemplo 2: Borrowing con referencia inmutable (&)
    let s3 = String::from("Mundo");
    let len = calcular_longitud(&s3); // Presta s3 como referencia inmutable
    println!("La longitud de '{}' es {}", s3, len);

    // Ejemplo 3: Borrowing con referencia mutable (&mut)
    let mut s4 = String::from("Hola");
    modificar_string(&mut s4); // Presta s4 como referencia mutable
    println!("s4 modificado: {}", s4);

    // Ejemplo 4: Evitar errores comunes con ownership
    let s5 = String::from("Rust");
    let s6 = &s5; // Presta s5 como referencia inmutable
    let s7 = &s5; // Puedes tener múltiples referencias inmutables
    // let s8 = &mut s5; // Esto daría un error: no se puede prestar como mutable mientras está prestado como inmutable
    println!("s5: {}, s6: {}, s7: {}", s5, s6, s7);

    // Ejemplo 5: Uso de referencias mutables exclusivas
    let mut s9 = String::from("Hola");
    {
        let s10 = &mut s9; // Presta s9 como referencia mutable
        s10.push_str(", mundo!");
    } // s10 sale de ámbito, el préstamo mutable termina
    let s11 = &mut s9; // Ahora puedes prestar s9 como mutable nuevamente
    s11.push_str(" ¡Adiós!");
    println!("s9: {}", s9);
}

// Función que toma una referencia inmutable (&) y devuelve la longitud del string
fn calcular_longitud(s: &String) -> usize {
    s.len()
}

// Función que toma una referencia mutable (&mut) y modifica el string
fn modificar_string(s: &mut String) {
    s.push_str(", mundo!");
}