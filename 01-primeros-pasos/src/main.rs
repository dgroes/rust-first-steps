fn main() {

    println!("--01--");
    let nombre = "Ferris";
    let patas = 8;

    println!("Me llamo {nombre} y tengo {patas} patas");
    println!("{} + {} = {}", 2, 3, 2 + 3);

    println!(" ");

    println!("--02--");
    const SEGUNDOS_POR_HORA: u32 = 60 * 60;
    const MAX_VIDAS: u32 = 3;

    println!("Una hora tiene {SEGUNDOS_POR_HORA} segundos");
    println!("Empiezas con {MAX_VIDAS} vidas");

    /*  */
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x vale {x}");

    /*  */
    let espacios = "    ";
    let espacios = espacios.len();
    println!("Hay {espacios} espacios");


    println!("--03--");
    let entero = 42;          // i32
    let decimal = 3.14;       // f64
    let activo = true;        // bool
    let puntos: u64 = 1_500;  // anotación explícita

    println!("{entero} {decimal} {activo} {puntos}");

    println!("i8:  {} a {}", i8::MIN, i8::MAX);
    println!("u8:  {} a {}", u8::MIN, u8::MAX);
    println!("i32: {} a {}", i32::MIN, i32::MAX);
    println!("u64: {} a {}", u64::MIN, u64::MAX);

    println!("");
    let dias: [&str; 3] = ["lunes", "martes", "miércoles"];
    let ceros = [0; 5]; // cinco ceros

    println!("El primero es {}", dias[0]);
    println!("Hay {} días", dias.len());
    println!("{:?}", ceros); //<- Imprime el array completo

}