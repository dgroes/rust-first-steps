# Comandos
- `cargo run`:	Compila y ejecuta el programa
- `cargo build`:	Compila sin ejecutar (en target/debug)
- `cargo build --release`:	Compila con optimizaciones para producción
- `cargo check`:	Comprueba que el código compila, sin generar el binario (muy rápido)

# Sujerencias
- Variables: Escribirlas en `snake_case`, si se usa `camelCase` el compilador 
- Variables que aun se usan: Por defecto no se pueden declara variables que no se usen, Cargo se quejará, pero se pueden definir con un guión bajo al inicio: `_temporal`
- Constantes (`const`): Escribirlas con `MAYÚSCULAS_CON_GUIONES_BAJOS`