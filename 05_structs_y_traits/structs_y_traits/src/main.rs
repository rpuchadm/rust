fn main() {
    // Crear una instancia de Persona usando el método estático
    let persona = Persona::nueva(String::from("Alice"), 30);

    // Llamar a un método de instancia
    persona.saludar(); // Imprime: "Hola, mi nombre es Alice y tengo 30 años."


    let persona = Persona::nueva(String::from("Bob"), 25);
    hacer_hablar(&persona); // Imprime: "Bob está hablando."

    let direccion = Direccion {
        calle: String::from("Calle Falsa 123"),
        ciudad: String::from("Springfield"),
    };

    let empleado = Empleado {
        persona: Persona::nueva(String::from("Charlie"), 40),
        direccion,
        salario: 50000,
    };

    println!("{} vive en {}", empleado.persona.nombre, empleado.direccion.ciudad);
}

// Definición del struct
struct Persona {
    nombre: String,
    edad: u8,
}

// Implementación de métodos para el struct
impl Persona {
    // Método de instancia
    fn saludar(&self) {
        println!("Hola, mi nombre es {} y tengo {} años.", self.nombre, self.edad);
    }

    // Método estático (constructor)
    fn nueva(nombre: String, edad: u8) -> Persona {
        Persona { nombre, edad }
    }
}

trait Hablador {
    fn hablar(&self);
}

impl Hablador for Persona {
    fn hablar(&self) {
        println!("{} está hablando.", self.nombre);
    }
}

fn hacer_hablar(hablador: &dyn Hablador) {
    hablador.hablar();
}

struct Direccion {
    calle: String,
    ciudad: String,
}

struct Empleado {
    persona: Persona,
    direccion: Direccion,
    salario: u32,
}
