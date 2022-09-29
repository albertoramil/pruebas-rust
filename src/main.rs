pub mod rectangulotype;
use std::fmt::format;
pub mod getapi;
pub mod usuariosrust;

use crate::rectangulotype::calculate_length;
use crate::rectangulotype::muestra;
use crate::rectangulotype::Coordenada;
use crate::rectangulotype::Rectangle;
pub mod triangulorectangulotype;

use crate::triangulorectangulotype::Coordenadatri;
use crate::triangulorectangulotype::Triangulo;
use crate::getapi::get;
use crate::usuariosrust::get_usuarios;
use crate::usuariosrust::crear_usuario;
use crate::usuariosrust::get_usuarioid;
use crate::usuariosrust::get_usuarioidupdate;


use crate::usuariosrust::actualizar_usuario;

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
        if entrada.accion == "triangulo" {

          println!("Triangulo datosssssssssss{:?} print!",  entrada.datos);

            println!("Triangulo {:?} print!", mitriangulo);
            println!("Area {:?} print!", mitriangulo.area());
            println!("Perimetro  triangulo{:?} print!", mitriangulo.perimeter());
        }else  if entrada.accion == "rectangulo" {
            println!("Rectangulo {:?} print!", mirectangulo2);
            println!("Area {:?} print!", mirectangulo2.area());
            println!("Perimetro {:?} print!", mirectangulo2.perimeter());
        }
        
        else  if entrada.accion == "json" {

            get();

        

        } else  if entrada.accion == "bd" {


            get_usuarios();
            crear_usuario();
            actualizar_usuario(3);
            get_usuarios();
            get_usuarioidupdate(5);
            get_usuarios();



        }
        else{
          println!("Incorrecto");
          println!("Vuelve intentarlo");

        }
    }
}

// para consumir un get

//curl -X GET 127.0.0.1:8181/ -H  "accept: application/json" -H  "Content-Type: application/json"
// use tokio::io::AsyncWriteExt;
// use tokio::net::{TcpListener, TcpStream};

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8181").await.unwrap();

//     loop {
//         let (stream, _) = listener.accept().await.unwrap();
//         handle_connection(stream).await;
//     }
// }

// async fn handle_connection(mut stream: TcpStream) {
//     let contents = "{\"mi respuesta\": respuesta chachita}";

//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
//         contents.len(),
//         contents
//     );
//     stream.write(response.as_bytes()).await.unwrap();
//     stream.flush().await.unwrap();
// }



// pub struct miRespuesta {
//     pub status: String,
//     pub body: String,
// }

// impl miRespuesta {
//     pub fn verDatos(algo: &Response) -> miRespuesta {
//         let palabra = String::from("estmomismo");
//         let r1 = &palabra; // no problem
//         let r2 = &palabra; // no problem

//         println!("Headers:\n{:#?}", algo.status());
//         println!("Headers:\n{:?}", algo.status());

//         miRespuesta {
//             status: String::from(r1),
//             body: String::from(r2),
//         }
//     }
// }

// // pub fn nueva(res:Response) -> miRespuesta {
// //   miRespuesta { status: "x", headers: "y",body:"fsfsd" }
// // }

// use reqwest::ClientBuilder;
// use reqwest::Response;
// use reqwest::Result;
// use std::io::Read;
// use std::time::Duration;

// use serde_json::Value;
// use std::error::Error;



     


