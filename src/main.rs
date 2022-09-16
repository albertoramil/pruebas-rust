

pub mod leer;
use std::fmt::format;

use crate::leer::muestra;


use crate::leer::calculate_length;


 fn main() {
    println!("mostrat {}", muestra());
  //  let palabra String ="estomismo";

    let palabra= String::from("estmomismo");





    let tamaño: usize= calculate_length(palabra);

    
    println!(" el tamaño de la palabra es:{}", tamaño.to_string());

    
    let i = 2;
    let a = "berzas";

    let salida = format!("teño {} {}", i, a);

    //let a = format!("{}", muestra());
    println!("{}", salida.to_string());
}








// fn main() {
//     let rectangle = Rectangle {
//         // Associated functions are called using double colons
//         p1: Point::origin(),
//         p2: Point::new(3.0, 4.0),
//     };

//     // Methods are called using the dot operator
//     // Note that the first argument `&self` is implicitly passed, i.e.
//     // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
//     println!("Rectangle perimeter: {}", rectangle.perimeter());
//     println!("Rectangle area: {}", rectangle.area());

//     let mut square = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(1.0, 1.0),
//     };

//     // Error! `rectangle` is immutable, but this method requires a mutable
//     // object
//     //rectangle.translate(1.0, 0.0);
//     // TODO ^ Try uncommenting this line

//     // Okay! Mutable objects can call mutable methods
//     square.translate(1.0, 1.0);

//     let pair = Pair(Box::new(1), Box::new(2));

//     pair.destroy();

//     // Error! Previous `destroy` call "consumed" `pair`
//     //pair.destroy();
//     // TODO ^ Try uncommenting this line
// }

