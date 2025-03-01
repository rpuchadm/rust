rustup show
rustup update
rustup check

Commands:
  show         Show the active and installed toolchains or profiles
  update       Update Rust toolchains and rustup
  check        Check for updates to Rust toolchains and rustup

---

rustc --version
cargo --version

---

mkdir 01_hello_world
cd 01_hello_world
cargo new hello_world

hello_world/
├── Cargo.toml
└── src/
    └── main.rs

cargo run

---

1. Manejo de errores
¿Cómo se manejan los errores en Rust con Result y Option?

¿Qué es el operador ? y cómo se usa para propagar errores?

Ejemplo práctico de manejo de errores en un programa.

2. Ownership y Borrowing
Explicación más detallada del sistema de propiedad (ownership) en Rust.

Diferencias entre &, &mut y move.

Ejemplos prácticos de cómo evitar errores comunes con el ownership.

3. Lifetimes (Tiempos de vida)
¿Qué son los lifetimes y por qué son importantes en Rust?

Ejemplos de cómo usar anotaciones de lifetimes en funciones y structs.

4. Colecciones
¿Cómo se usan las colecciones como Vec, HashMap y HashSet?

Ejemplos prácticos de operaciones comunes con colecciones.

5. Traits y Genéricos
¿Cómo se definen y usan los traits en Rust?

¿Qué son los genéricos y cómo se usan para escribir código reutilizable?

Ejemplo de un trait personalizado y su implementación.

6. Concurrencia y Paralelismo
¿Cómo se maneja la concurrencia en Rust con threads y async/await?

Ejemplos de cómo usar Mutex, Arc y canales (mpsc).

Introducción a tokio y async-std para programación asíncrona.

7. Macros
¿Qué son las macros en Rust y cómo se usan?

Ejemplos de macros comunes como println!, vec! y cómo crear tus propias macros.

8. Testing (Pruebas)
¿Cómo se escriben y ejecutan pruebas en Rust?

Ejemplos de pruebas unitarias y de integración.

9. Interoperabilidad con C
¿Cómo se llama a funciones de C desde Rust y viceversa?

Ejemplo de uso de bindgen para generar enlaces a bibliotecas de C.

10. Proyectos más grandes
¿Cómo se estructura un proyecto grande en Rust?

Uso de workspaces en Cargo para manejar múltiples crates en un solo proyecto.

11. Rust avanzado
¿Qué son los unsafe blocks y cuándo se usan?

Introducción a los closures y cómo se usan en Rust.

Ejemplos de patrones avanzados como RAII y type-state pattern.

12. Ecosistema de Rust
¿Cuáles son las bibliotecas más populares en Rust (crates)?

Introducción a frameworks como Rocket (para web) y Actix.

Uso de herramientas como clippy, rustfmt y cargo watch.

13. Rust en producción
¿Cómo se despliega una aplicación Rust en producción?

Uso de Docker con Rust.

Optimización de código Rust para rendimiento.

14. Proyectos prácticos
Crear una API REST simple con Rust.

Desarrollar una CLI (interfaz de línea de comandos) con Rust.

Crear un juego simple usando un motor gráfico como ggez o bevy.

15. Comunidad y recursos
¿Dónde puedo encontrar más recursos para aprender Rust?

¿Cuáles son las mejores prácticas recomendadas por la comunidad de Rust?

---

~/.cargo

