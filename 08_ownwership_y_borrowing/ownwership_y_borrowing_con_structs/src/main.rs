#[derive(Debug)]
struct Contador {
    valor: i32,
}

impl Contador {
    // Método que toma una referencia mutable a self y modifica el valor
    fn incrementar(&mut self) -> i32 {
        self.valor += 1;
        self.valor
    }
}

// Función que toma un Contador como referencia mutable y lo modifica
fn incrementar_contador(contador: &mut Contador) -> i32 {
    contador.incrementar()
}

// Función que toma ownership de un Contador y lo devuelve modificado
fn tomar_ownership_y_incrementar(mut contador: Contador) -> Contador {
    contador.incrementar();
    contador
}

fn main() {
    // Crear una instancia de Contador
    let mut contador = Contador { valor: 0 };

    // Incrementar el contador usando una referencia mutable
    let nuevo_valor = incrementar_contador(&mut contador);
    println!("Valor después de incrementar: {}", nuevo_valor); // Valor: 1

    // Incrementar el contador usando su método
    let nuevo_valor = contador.incrementar();
    println!("Valor después de incrementar: {}", nuevo_valor); // Valor: 2

    // Tomar ownership del contador y modificarlo
    let contador = tomar_ownership_y_incrementar(contador);
    println!("Valor después de tomar ownership: {}", contador.valor); // Valor: 3

    // Intentar usar el contador después de moverlo (esto daría un error)
    // let nuevo_valor = contador.incrementar(); // Error: value borrowed here after move
}