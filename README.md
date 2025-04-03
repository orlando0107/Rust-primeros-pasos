# Comandos Básicos de Rust

Este documento contiene los comandos más utilizados en Rust y una breve explicación de cada uno.

## Comandos Fundamentales

### Creación y Gestión de Proyectos

- `cargo new nombre_proyecto` - Crea un nuevo proyecto de Rust
- `cargo init` - Inicializa un nuevo proyecto en el directorio actual
- `cargo build` - Compila el proyecto
- `cargo run` - Compila y ejecuta el proyecto
- `cargo check` - Verifica si el código compila sin generar el ejecutable
- `cargo test` - Ejecuta las pruebas del proyecto
- `cargo clean` - Elimina los archivos compilados
- `cargo update` - Actualiza las dependencias del proyecto

### Gestión de Dependencias

- `cargo add nombre_paquete` - Añade una dependencia al proyecto
- `cargo remove nombre_paquete` - Elimina una dependencia del proyecto
- `cargo search nombre_paquete` - Busca paquetes en crates.io

### Documentación

- `cargo doc` - Genera la documentación del proyecto
- `cargo doc --open` - Genera y abre la documentación en el navegador

### Publicación

- `cargo publish` - Publica el paquete en crates.io
- `cargo login token` - Inicia sesión en crates.io

### Herramientas de Desarrollo

- `rustup update` - Actualiza Rust a la última versión
- `rustup show` - Muestra la información de instalación de Rust
- `rustc --version` - Muestra la versión del compilador de Rust
- `cargo --version` - Muestra la versión de Cargo

## Consejos Adicionales

- Usa `cargo build --release` para compilar en modo de optimización
- Usa `cargo run --release` para ejecutar en modo de optimización
- Usa `cargo fmt` para formatear el código según las convenciones de Rust
- Usa `cargo clippy` para ejecutar el linter de Rust
