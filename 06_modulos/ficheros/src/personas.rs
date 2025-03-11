// personas.rs

#[derive(Debug)]
pub struct Persona {
    pub nombre: String,
    pub edad: u8,
}

impl Persona {
    pub fn new(nombre: String, edad: u8) -> Self {
        Persona { nombre, edad }
    }
}

pub fn mostrar_persona(persona: &Persona) {
    println!("Nombre: {}, Edad: {}", persona.nombre, persona.edad);
}
