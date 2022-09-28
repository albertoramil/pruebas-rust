



extern crate rust_comienzo;
extern crate diesel;

use self::rust_comienzo::*;
use self::models::*;
use self::diesel::prelude::*;

use self::models::{Usuariosrust};



pub fn get_usuarios(){
    let connection = &mut establish_connection();
    use self::schema::usuariosrust::dsl::*;
    let results = usuariosrust       
        .limit(5)
        .load::<Usuariosrust>(connection)
        .expect("Error obteniendo los usuarios");


        for usuario in results{
           
            println!("Nombre usuario: {}, apellidos: {} ",usuario.nombre,usuario.apellidos);
        }
    
}




// pub fn create_player(usuariosrust: &Usuariosrust) -> Result<usize, diesel::result::Error> {
//     use self::schema::usuariosrust::dsl::*;
//     let conn = &mut establish_connection();
    
//     let nombre = String::from("estmomismo");
//     let apellidos = String::from("estmomismo");
//     let id: i32 = 3;


//     let nuevo_usuario = Usuariosrust {id,nombre, apellidos };

//     diesel::insert_into(usuariosrust::table)
//         .values(nuevo_usuario)
//         .execute(conn)
// }