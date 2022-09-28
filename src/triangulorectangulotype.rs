use std::string;

#[derive(Debug)]

// creo el objeto coordenada
pub struct Coordenadatri {
    pub x: f64,
    pub y: f64,
}

//todos los metodos para los objetos coordenada que comparten la struct anterior
impl Coordenadatri {
    //funcion origen que devuelve un objeto coordenada, en este caso el (0,0)
    pub fn origen() -> Coordenadatri {
        Coordenadatri { x: 0.0, y: 0.0 }
    }

    //funcion nueva coordenada que devuelve un objeto coordenada, con la pos que nos de la gana
    //pasando como los valores tipo binary64
    pub fn nueva(x: f64, y: f64) -> Coordenadatri {
        Coordenadatri { x: x, y: y }
    }
}

#[derive(Debug)]

//objeto hereda del objeto coordenada por ellos tb tendrá todos sus metodos
pub struct Triangulo {
    pub p1: Coordenadatri,
    pub p2: Coordenadatri,
}

impl Triangulo {
    pub fn area(&self) -> f64 {
        let Coordenadatri { x: x1, y: y1 } = self.p1;
        let Coordenadatri { x: x2, y: y2 } = self.p2;

        (((x2) * (y2)) / 2.0).abs()
    }

    //self para obtener los parametros del constructor
    pub fn perimeter(&self) -> f64 {
        let Coordenadatri { x: x1, y: y1 } = self.p1;
        let Coordenadatri { x: x2, y: y2 } = self.p2;

        //realmente sobra  el x1 e y1 porque siempre son 0 ambos
        (((x1 - x2).abs() * (x1 - x2).abs() + (y1 - y2).abs() * (y1 - y2).abs()).sqrt()) + x2 + y2
    }
}
