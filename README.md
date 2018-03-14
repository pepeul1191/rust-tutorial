## RUST 

Instalación de Rust y Cargo

    $ curl -sSf https://static.rust-lang.org/rustup.sh | sh

Nuevo proyecto Rust con Cargo:

    $ cargo new <nombre-proyecto> --bin

Correr aplicación Rust usando Cargo

    $ cargo run

Autoreload de cargo ante cambios:

    $ cargo install cargo-watch

Para ejecutar el autoreolad:

    $ cargo watch -x run

---

Fuentes:

+ http://nickel-org.github.io/getting-started.html
+ https://doc.rust-lang.org/book/first-edition/crates-and-modules.html
+ https://github.com/passcod/cargo-watch