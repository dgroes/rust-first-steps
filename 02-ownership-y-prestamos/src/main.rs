fn main() {
    //EJERCICIOS PARA ENTENDER EL OWNERSHIP
    println!("--OWNERSHIP--");
    
    /* Objetivo: Tienes dos variables que necesitan mantener versiones distintas de un mismo mensaje en el heap. Haz que el código compile sin borrar ningún println!, asegurándote de que cada variable sea dueña de su propia memoria. */

    /* ```rust
        fn main() {
        let mensaje_original = String::from("Hola Rust");

        // TODO: Arregla esta línea para que mensaje_original siga siendo válido
        let mut mensaje_editado = mensaje_original;

        mensaje_editado.push_str("acean!");

        println!("Original: {mensaje_original}");
        println!("Editado: {mensaje_editado}");
    }

    ``` */

    fn one() {
        let mensaje_original = String::from("Hola Rust");
        let mut mensaje_editado = mensaje_original.clone();

        mensaje_editado.push_str("acean!");

        println!("Original: {mensaje_original}");
        println!("Editado: {mensaje_editado}");
    }

    /* Objetivo: En este programa, el paquete pasa por dos estaciones de control. Como todavía no sabes usar préstamos (&), las funciones deben recibir la propiedad del String, modificarlo y devolverlo para que la siguiente función (o el main) pueda seguir usándolo. Prohibido usar .clone().

    SALIDA: Estado final: Caja [Embalada] -> Destino: Chile
    */

    fn two() {
        let paquete = String::from("Caja");

        // 1. Pasamos la propiedad a 'embalar' y capturamos el String devuelto.
        // Usamos 'shadowing' (volver a usar 'let paquete') o puedes usar 'let mut paquete'.
        let paquete = embalar(paquete);

        // 2. Pasamos la propiedad actualizada a 'etiquetar' y recuperamos el resultado final.
        let paquete = etiquetar(paquete);

        println!("Estado final: {paquete}");
    }

    // Añadimos `-> String` en la firma para indicar que la función devuelve la propiedad
    fn embalar(mut item: String) -> String {
        item.push_str(" [Embalada]");
        item // Sin punto y coma al final para retornar el valor
    }

    // Recibe el String, lo vuelve mutable en su ámbito, lo modifica y lo devuelve
    fn etiquetar(mut item: String) -> String {
        item.push_str(" -> Destino: Chile");
        item
    }

    /*

    Objetivo: Este ejercicio combina control de flujo (for o while), tipos Copy (u32), tuplas y ownership. Debes crear una función censurar que tome un String y un carácter prohibido (char), y lo reemplace por un asterisco '*' mientras cuenta cuántas letras censuró. Como main necesita imprimir la palabra censurada y el conteo al final, la función debe devolver ambos datos en una tupla. Sin usar .clone().

    (Pista: puedes recorrer los caracteres de un String usando for c in texto.chars() y construir un nuevo String vacío con String::new()).


    SALIDA ESPERADA: Palabra censurada: b*n*n* (se ocultaron 3 letras 'a')
    */

    fn three() {
        let palabra = String::from("banana");
        let letra_prohibida = 'a';

        // TODO: Llama a censurar y recoge la tupla resultante
        let (palabra_censurada, cantidad) = censurar(palabra, letra_prohibida);

        // TODO: Haz que este println! funcione usando las variables correctas
        println!(
            "Palabra censurada: {palabra_censurada} (se ocultaron {cantidad} letras '{letra_prohibida}')"
        );
    }

    // TODO: Completa la función. Debe recibir el String y el char,
    // devolver una tupla (String, u32) con el nuevo texto y el total de reemplazos.
    fn censurar(texto: String, prohibida: char) -> (String, u32) {
        let mut nuevo_texto = String::new();
        let mut contador = 0;

        // Escribe aquí tu bucle para revisar cada carácter

        for c in texto.chars() {
            if c == prohibida {
                nuevo_texto.push_str("*");
                contador += 1;
            } else {
                // Añadir la letra a nuevo texto (primera vuelta: B; segunda vuelta: *; tercera vuelta: n, etc.)
                nuevo_texto.push(c);
            }

            
        }

        (nuevo_texto, contador)
    }

    one();
    two();
    three();

    // REFERENCIA
    println!("");
    println!("--REFERENCIA--");

    fn prestamos() {
        let s1 = String::from("hola");
        let longitud = calcular_longitud(&s1); // prestamos s1
        println!("«{s1}» tiene {longitud} bytes"); // s1 sigue siendo válida
    }

    fn calcular_longitud(s: &String) -> usize {
        s.len()
    } // s sale de ámbito, pero no es dueña de nada: no se libera nada
    
    prestamos();




    fn prestamos_cambio() {
        let mut s = String::from("hola");
        cambiar(&mut s);
        println!("{s}");
    }

    fn cambiar(texto: &mut String) {
        texto.push_str(", mundo");
    }

    prestamos_cambio();

}
