use std::io::{self, Write};


use std::fmt::format;






pub struct Command {
    pub accion: String,
    pub datos: String,

}

impl Command {
    pub fn new() -> Command {
        Command {
            accion: String::new(),
            datos: String::new(),
        }
    }

    fn parse(&mut self, input_str: &str) {
        let mut split_input_iter = input_str.trim().split_whitespace();
        // split_input_iter trae la respuesta entera
        // SplitWhitespace { inner: Filter { iter: Split(SplitInternal { start: 0, end: 9, matcher: CharPredicateSearcher { haystack: "triangulo", char_indices: CharIndices { front_offset: 0, iter: Chars(['t', 'r', 'i', 'a', 'n', 'g', 'u', 'l', 'o']) } }, allow_trailing_empty: true, finished: false }) } } print!
        //Triangulo Triangulo { p1: Coordenadatri { x: 0.0, y: 0.0 }, p2: Coordenadatri { x: 2.0, y: 2.0 } }

        self.accion = split_input_iter.next().unwrap_or_default().to_string(); //para que devuela al menu la accion
   
        self.datos = split_input_iter.next().unwrap_or_default().to_string();

    }
}

pub fn get_input() -> Command {
    // prompt
    println!("");
    print!("De que calcular los datos, triangulo, rectangulo o json> ");

    io::stdout().flush().unwrap(); // si no tolea con el orden de los que saca

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Error leyendo...");
    println!("-----------entradaentrada------ {:?} print!", entrada);

    println!("");

    // parse
    let mut command = Command::new();
    //se crea el nuevo comando con la entrada y listo
    command.parse(entrada.as_str());

    // return
    command
}
