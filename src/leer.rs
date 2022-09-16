use std::string;

// creo el objeto coordenada
struct Coordenada {
    x: f64,
    y: f64,
}

//todos los metodos para los objetos coordenada que comparten la struct anterior
impl Coordenada {
    //funcion origen que devuelve un objeto coordenada, en este caso el (0,0)
    pub fn origen() -> Coordenada {
        Coordenada { x: 0.0, y: 0.0 }
    }

    //funcion nueva coordenada que devuelve un objeto coordenada, con la pos que nos de la gana
    //pasando como los valores tipo binary64
    pub fn nueva(x: f64, y: f64) -> Coordenada {
        Coordenada { x: x, y: y }
    }
}
//objeto hereda del objeto coordenada por ellos tb tendrá todos sus metodos
struct Rectangle {
    p1: Coordenada,
    p2: Coordenada,
}




//implementamos medotos para el objeto rectangulo, utilizando los que hereda
impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`

    pub fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Coordenada { x: x1, y: y1 } = self.p1;
        let Coordenada { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }
    //self para obtener los parametros del constructor
    pub fn perimeter(&self) -> f64 {
        let Coordenada { x: x1, y: y1 } = self.p1;
        let Coordenada { x: x2, y: y2 } = self.p2;

        //realmente sobra  el x1 e y1 porque siempre son 0 ambos
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }
}

pub fn calculate_length(algo: String) -> usize {
    let s = String::from(algo);

    s.len()
}

pub fn muestra() -> String {
    
    let s = String::from("hello");

    s
}
