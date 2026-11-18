# C01: Comandos
- `cargo run`:	Compila y ejecuta el programa
- `cargo build`:	Compila sin ejecutar (en target/debug)
- `cargo build --release`:	Compila con optimizaciones para producción
- `cargo check`:	Comprueba que el código compila, sin generar el binario (muy rápido)
# C02: Sujerencias
- Variables: Escribirlas en `snake_case`, si se usa `camelCase` el compilador 
- Variables que aun se usan: Por defecto no se pueden declara variables que no se usen, Cargo se quejará, pero se pueden definir con un guión bajo al inicio: `_temporal`
- Constantes (`const`): Escribirlas con `MAYÚSCULAS_CON_GUIONES_BAJOS`
# C03: Reglas
- Usar siempre punto y coma `;`
- Las comillas simples son para los tipos de dato `char`, las comillas doble para los `strig`
- Las tuplas pueden guardar valores de distintos tipos
- Los arrays pueden guardar solo valores del mismo tipo con una lonigud fija.  
# C04: Anotaciones de Tipos en Rust
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
# C05: Tuplas y Arrays en Rust
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
# C06: Bucles en Rust (`loop`, `while`, `for`)
Rust ofrece tres estructuras principales para repetir bloques de código según tus necesidades de control y rendimiento.

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
## Resumen de Referencia

| Estructura | Cuándo usarla | Mecanismo de parada |
| :--- | :--- | :--- |
| **`loop`** | Bucle infinito o cuando necesitas que el bucle devuelva un valor. | Explícito (`break valor`) |
| **`while`** | Cuando la repetición depende de que una condición booleana sea verdadera. | Condición evaluada a `false` |
| **`for`** | Cuando quieres recorrer colecciones o rangos definidos de principio a fin. | Fin de la colección / rango |
# C07: Ownership (Propiedad)

El sistema de **Ownership** (propiedad) es la característica más única de Rust y es lo que permite garantizar la seguridad de la memoria **sin necesidad de recolector basura** (*garbage collector*).

## 1. Memoria: Stack vs. Heap

Para entender por qué existe el *Ownership*, primero hay que entender dónde guarda la computadora la información de tus programas. La memoria RAM se divide en dos zonas principales con reglas muy diferentes:

### El Stack (La Pila): Datos rápidos y ordenados
Imagina una **pila de platos** en una cafetería: el último plato que pones encima es el primero que sacas (*LIFO*: Último en entrar, primero en salir).
- **Cómo funciona:** Es una zona de memoria ultra rápida.
- **La trampa:** Todo lo que guardes aquí **debe tener un tamaño fijo y conocido en tiempo de compilación** (antes de ejecutar el programa).
- **Ejemplos:** Un número como `i32`, un booleano `bool`, un carácter `char` o un array de tamaño fijo.
- **Ciclo de vida:** Las variables que viven en el Stack aparecen cuando entra una función y **desaparecen automáticamente** de forma limpia en cuanto la función termina.

### El Heap (El Montón): El depósito flexible pero caótico
Imagina un **almacén gigante** donde la gente guarda cajas de cualquier tamaño. Sirve para almacenar datos **cuyo tamaño puede cambiar o es desconocido en tiempo de compilación**.
- **Cómo funciona:** Cuando necesitas guardar algo dinámico (como un `String` largo o un `Vec`), le pides espacio al sistema. El sistema busca un hueco libre, lo guarda y te entrega un **puntero** (una dirección en el Stack que apunta a dónde están los datos reales en el Heap).
- **La trampa:** Leer y escribir aquí es más lento. **El mayor problema es la limpieza:** si pides espacio en el Heap, alguien tiene que encargarse de borrarlo cuando ya no sirva (si no lo borras hay fugas de memoria; si lo borras dos veces, corrompes los datos).

> **El gran dilema que resuelve Rust:** No usa recolector de basura (como Java/Python) ni te deja limpiar la memoria manualmente (como C/C++). En su lugar, el compilador aplica las reglas de **Ownership** para liberar la memoria del Heap automáticamente cuando el dueño muere.
## 2. Las Tres Reglas de Oro de la Propiedad

Para entender cómo maneja Rust la memoria, solo debes seguir estas tres reglas fundamentales:

1. Cada valor en Rust tiene un **propietario** (una variable que es dueña de ese valor).
2. Solo puede haber **un propietario a la vez**.
3. Cuando el propietario sale del ámbito (*scope*), el valor se **destruye** (se libera la memoria automáticamente).

```rust
fn main() {
    {
        let s = String::from("hola"); // 's' es el propietario del valor
        println!("{s}");
    } // <- Aquí termina el ámbito. 's' sale de alcance y Rust libera la memoria automáticamente.
    
    // println!("{s}"); // Error: 's' ya no existe aquí
}
```


## 3. Movimiento de Datos (*Move*)

Cuando asignas una variable que vive en el *Heap* (como un `String`) a otra, **la propiedad se transfiere (se mueve)**. La variable original deja de ser válida.

```rust
fn main() {
    let s1 = String::from("rust"); // 's1' apunta a los datos en el Heap
    let s2 = s1;                   // La propiedad se MUEVE a 's2'. 's1' queda invalidada.

    // println!("{s1}"); // ❌ Error: valor usado después de un 'move'
    println!("{s2}");       // ✅ Funciona perfectamente
}
```

- **¿Por qué hace esto Rust?** Para evitar el *double free* (liberar la misma memoria dos veces al terminar el programa), lo cual causaría fallos de seguridad o corrupción de datos.

## 4. Clonación (*Clone*)

Si necesitas conservar el valor original y además crear una copia independiente en el *Heap*, debes solicitarlo explícitamente usando el método `.clone()`.

```rust
fn main() {
    let s1 = String::from("rust");
    let s2 = s1.clone(); // Se duplica todo el contenido en el Heap para 's2'

    println!("s1 = {s1}, s2 = {s2}"); // ✅ Ambos funcionan porque son independientes
}
```

- **Costo:** Duplicar datos en el *Heap* consume más recursos y tiempo de CPU, por lo que Rust te obliga a escribir `.clone()` explícitamente.

## 5. Tipos que se Copian en el *Stack* (`Copy`)

Los tipos de datos simples de tamaño conocido en tiempo de compilación (como los enteros `i32`, los booleanos `bool`, o los caracteres `char`) se almacenan por completo en el *Stack*. 

En estos tipos, la asignación **copia** los datos automáticamente en lugar de moverlos, por lo que la variable original sigue viva.

```rust
fn main() {
    let x: i32 = 5; // 'x' vive completamente en el Stack
    let y = x;      // Se copia el valor en otra posición del Stack. 'x' sigue siendo válido.

    println!("x = {x}, y = {y}"); // ✅ ¡Funciona sin problemas!
}
```

## 6. Ownership en Funciones

Cuando le pasas una variable propietaria de datos del *Heap* a una función, **le transfieres la propiedad** igual que en una asignación. Una vez que la función termina, esa variable se destruye y ya no la puedes usar en el `main`.

```rust
fn procesar_texto(texto: String) {
    println!("Procesando: {texto}");
} // <-- 'texto' sale de ámbito aquí y la memoria del Heap es liberada automáticamente.

fn main() {
    let mi_mensaje = String::from("Aprender Rust");

    procesar_texto(mi_mensaje); // La propiedad se mueve de 'mi_mensaje' a la función.

    // ❌ Error: 'mi_mensaje' ya no pertenece al main, su memoria fue liberada.
    // println!("{mi_mensaje}"); 
}
```

## Resumen Clave

| Concepto | Qué ocurre | Zona de Memoria | Tipos típicos |
| :--- | :--- | :--- | :--- |
| **Move (Mover)** | Transfiere la propiedad; la variable original expira. | Heap | `String`, vectores (`Vec`), structs complejos. |
| **Clone (Clonar)** | Duplica los datos explícitamente en memoria. | Heap | Cualquier tipo que implemente `Clone`. |
| **Copy (Copiar)** | Copia implícita rápida; la original sigue viva. | Stack | Enteros (`i32`), floats (`f64`), `bool`, `char`. |
# C08:
# C09:
# C10:
# C11:
# C12:
# C13:
# C14:
# C15:
# C16: