# Comandos
- `cargo run`:	Compila y ejecuta el programa
- `cargo build`:	Compila sin ejecutar (en target/debug)
- `cargo build --release`:	Compila con optimizaciones para producción
- `cargo check`:	Comprueba que el código compila, sin generar el binario (muy rápido)

# Sujerencias
- Variables: Escribirlas en `snake_case`, si se usa `camelCase` el compilador 
- Variables que aun se usan: Por defecto no se pueden declara variables que no se usen, Cargo se quejará, pero se pueden definir con un guión bajo al inicio: `_temporal`
- Constantes (`const`): Escribirlas con `MAYÚSCULAS_CON_GUIONES_BAJOS`

# Reglas
- Usar siempre punto y coma `;`
- Las comillas simples son para los tipos de dato `char`, las comillas doble para los `strig`
- Las tuplas pueden guardar valores de distintos tipos
- Los arrays pueden guardar solo valores del mismo tipo con una lonigud fija.  

# Anotaciones de Tipos en Rust
## 1. Numéricos
- **Enteros con signo (`i`)**: `i8`, `i16`, `i32` (por defecto), `i64`, `i128`, `isize` (dependiente de la arquitectura).
- **Enteros sin signo (`u`)**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (usado para índices y tamaños).
- **Flotantes**: `f32`, `f64` (por defecto para decimales).

```rust
let edad: i32 = 30;
let cantidad: usize = 10;
let temperatura: f64 = 23.5;
```

## 2. Primitivos Básicos
- **Boleano (`bool`)**: `true` o `false`.
- **Caracter (`char`)**: Carácter Unicode de 4 bytes (comillas simples).

```rust
let activo: bool = true;
let letra: char = 'R';
```

## 3. Cadenas de Texto
- **`&str`**: Referencia a una cadena de texto (slice).
- **`String`**: Cadena dinámica con propiedad (en el *heap*).

```rust
let saludo: &str = "Hola, mundo";
let nombre: String = String::from("Diego");
```

## 4. Colecciones
- **Array (`[T; N]`)**: Tamaño fijo.
- **Tupla `(T1, T2)`**: Agrupación de tipos mixtos fijos.
- **Vector (`Vec<T>`)**: Lista dinámica.

```rust
let notas: [i32; 3] = [9, 8, 10];
let punto: (i32, f64) = (5, 3.14);
let lenguajes: Vec<String> = vec![String::from("Rust")];
```

## 5. Opcionales y Resultados (`Option` y `Result`)
- **`Option<T>`**: `Some(T)` o `None` (manejo de nulos).
- **`Result<T, E>`**: `Ok(T)` o `Err(E)` (manejo de errores).

```rust
let id_usuario: Option<i32> = Some(42);
let respuesta: Result<f64, &str> = Ok(9.8);
```

## 6. Funciones
- Requieren tipado explícito en parámetros y flecha `->` para el retorno.

```rust
fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
```
# Tuplas y Arrays en Rust

## 1. Tuplas `(T1, T2, ...)`
- **Características**: Longitud fija y permiten almacenar valores de **distintos tipos** en una misma estructura. Se almacenan en el *stack*.
- **Acceso**: Mediante índice con punto (`tupla.0`) o por desestructuración.

```rust
// Declaración con anotación de tipos
let usuario: (&str, i32, bool) = ("Diego", 28, true);

// Acceso por índice
let nombre = usuario.0; // "Diego"

// Desestructuración
let (n, edad, activo) = usuario;
```

## 2. Arrays `[T; N]`
- **Características**: Longitud fija conocida en tiempo de compilación y todos sus elementos deben ser del **mismo tipo** (`T`). Se almacenan en el *stack*.
- **Acceso**: Mediante corchetes (`array[0]`). Rust valida en tiempo de ejecución que no te salgas de los límites (*panic* si ocurre).

```rust
// Declaración explícita: [tipo; cantidad]
let numeros: [i32; 4] = [10, 20, 30, 40];

// Inicializar todos los elementos con el mismo valor: [valor; cantidad]
let ceros: [i32; 5] = [0; 5]; // [0, 0, 0, 0, 0]

// Acceso y longitud
let primero = numeros[0]; // 10
let largo = numeros.len(); // 4
```

## 3. Diferencia Clave
| Característica | Tupla `(T1, T2)` | Array `[T; N]` |
| :--- | :--- | :--- |
| **Tipos de datos** | Heterogéneos (distintos tipos) | Homogéneos (mismo tipo) |
| **Tamaño** | Fijo | Fijo |
| **Iteración** | No es directamente iterable | Iterable (`for x in arr`) |

# Bucles en Rust (`loop`, `while`, `for`)

Rust ofrece tres estructuras principales para repetir bloques de código según tus necesidades de control y rendimiento.

---

## 1. `loop` (Bucle Infinito)
- **Uso**: Repite un bloque de código indefinidamente hasta que indiques de forma explícita cuándo salir mediante `break`.
- **Característica clave**: A diferencia de otros lenguajes, en Rust `loop` puede **devolver un valor** al usar `break valor;`.

```rust
fn main() {
    let mut contador = 0;

    let resultado = loop {
        contador += 1;
        if contador == 5 {
            break contador * 2; // Rompe el bucle y asigna 10 a 'resultado'
        }
    };

    println!("Resultado: {resultado}");
}
```

---

## 2. `while` (Bucle Condicional)
- **Uso**: Evalúa una condición booleana al inicio de cada iteración. Si es `true`, ejecuta el bloque; si es `false`, el bucle termina.
- **Ideal para**: Casos en los que no sabes cuántas veces se repetirá el proceso, pero depende de que una condición cambie.

```rust
fn main() {
    let mut n = 3;

    while n > 0 {
        println!("{n}...");
        n -= 1;
    }

    println!("¡Despegue!");
}
```

---

## 3. `for` (Bucle Iterativo y Seguro)
- **Uso**: Itera de forma segura sobre una secuencia de elementos (rangos, arrays, vectores). Es el bucle más utilizado en Rust porque elimina el riesgo de errores por índices fuera de límites.
- **Rangos**: Se definen con `..` (excluye el límite superior) o `..=` (incluye el límite superior).

```rust
fn main() {
    // Iterar en un rango del 1 al 3
    for i in 1..=3 {
        println!("Número: {i}");
    }

    // Iterar sobre elementos de un array
    let lenguajes = ["Rust", "Python", "JavaScript"];
    for lang in lenguajes {
        println!("Lenguaje: {lang}");
    }
}
```

---

## 4. Etiquetas de Bucle (`'etiqueta:`)
- **Uso**: Permiten controlar bucles exteriores desde un bucle anidado usando `break 'etiqueta` o `continue 'etiqueta`.
- **Sintaxis**: Comilla simple seguida del nombre y dos puntos (`'externo:`).

```rust
fn main() {
    'externo: for fila in 1..=3 {
        for columna in 1..=3 {
            if fila * columna == 4 {
                break 'externo; // Sale de ambos bucles de golpe
            }
        }
    }
}
```

---

## Resumen de Referencia

| Estructura | Cuándo usarla | Mecanismo de parada |
| :--- | :--- | :--- |
| **`loop`** | Bucle infinito o cuando necesitas que el bucle devuelva un valor. | Explícito (`break valor`) |
| **`while`** | Cuando la repetición depende de que una condición booleana sea verdadera. | Condición evaluada a `false` |
| **`for`** | Cuando quieres recorrer colecciones o rangos definidos de principio a fin. | Fin de la colección / rango |