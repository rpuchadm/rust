// Declarar los submódulos
pub mod saludos;
pub mod matematicas;

// Reexportar las funciones para facilitar su uso
pub use saludos::saludar;
pub use matematicas::sumar;