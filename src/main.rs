pub mod rectangulotype;
use std::fmt::format;

use crate::rectangulotype::muestra;
use crate::rectangulotype::calculate_length;
use crate::rectangulotype::Rectangle;
use crate::rectangulotype::Coordenada;
pub mod triangulorectangulotype;


use crate::triangulorectangulotype::Coordenadatri;
use crate::triangulorectangulotype::Triangulo;


pub mod rlib;





fn main() {
    println!("mostrat {}", muestra());
    //  let palabra String ="estomismo";

    let palabra = String::from("estmomismo");

    let tamaño: usize = calculate_length(palabra);

    println!(" el tamaño de la palabra es:{}", tamaño.to_string());

    let i = 2;
    let a = "berzas";

    let salida = format!("teño {} {}", i, a);

    //let a = format!("{}", muestra());
    println!("{}", salida.to_string());

    //************************************* */
    let micoordenada1: Coordenada = Coordenada { x: (0.0), y: (1.1) };
    println!("Coordenada 1 {:?} print!", micoordenada1);
    let micoordenada2: Coordenada = Coordenada { x: (3.3), y: (4.4) };
    println!("Coordenada 2 {:?} print!", micoordenada2);

    //************************************* */

    let mirectangulo: Rectangle = Rectangle {
        p1: (micoordenada1),
        p2: (micoordenada2),
    };
    println!("Rectangulo {:?} print!", mirectangulo);
    println!("Area {:?} print!", mirectangulo.area());
    println!("Perimetro {:?} print!", mirectangulo.perimeter());

    //************************************* */

    let micoordenada3 = Coordenada::origen();
    println!("Coordenada creada {:?} print!", micoordenada3);
    let micoordenada4 = Coordenada::nueva(2.0, 6.0);
    println!("Coordenada creada {:?} print!", micoordenada4);

    //************************************* */

    let mirectangulo2: Rectangle = Rectangle {
        p1: Coordenada::origen(),
        p2: Coordenada::nueva(5.0, 8.0),
    };
    println!("Rectangulo {:?} print!", mirectangulo2);
    println!("Area {:?} print!", mirectangulo2.area());
    println!("Perimetro {:?} print!", mirectangulo2.perimeter());


    //********************** */
    let mitriangulo: Triangulo = Triangulo {
      p1: Coordenadatri::origen(),
      p2: Coordenadatri::nueva(2.0, 2.0),
  };
  println!("Triangulo {:?} print!", mitriangulo);
  println!("Area {:?} print!", mitriangulo.area());
  println!("Perimetro  triangulo{:?} print!", mitriangulo.perimeter());

 

let mut entrada = rlib::Command::new();

while entrada.accion != "salir" {
  entrada = rlib::get_input();
if  entrada.accion == "triangulo" {
  println!("Triangulo {:?} print!", mitriangulo);
  println!("Area {:?} print!", mitriangulo.area());
  println!("Perimetro  triangulo{:?} print!", mitriangulo.perimeter());}



if  entrada.accion == "rectangulo" {
  println!("Rectangulo {:?} print!", mirectangulo2);
  println!("Area {:?} print!", mirectangulo2.area());
  println!("Perimetro {:?} print!", mirectangulo2.perimeter());}
 


}



}
