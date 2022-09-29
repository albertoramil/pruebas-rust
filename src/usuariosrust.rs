extern crate diesel;
extern crate rust_comienzo;

use self::diesel::prelude::*;
use self::models::*;
use self::rust_comienzo::*;

use self::models::Usuariosrust;

pub fn get_usuarios() {
    let connection = &mut establish_connection();
    use self::schema::usuariosrust::dsl::*;
    let results = usuariosrust
        .limit(5)
        .load::<Usuariosrust>(connection)
        .expect("Error obteniendo los usuarios");

    for usuario in results {
        println!(
            "Nombre usuario: {}, apellidos: {} ",
            usuario.nombre, usuario.apellidos
        );
    }
}

// pub fn create_usuariosrust() -> Result<usize, diesel::result::Error> {
//     use self::schema::usuariosrust::dsl::*;
//     let conn = &mut establish_connection();

//     let self::schema::usuariosrust::nombre = String::from("titooooo");
//     let self::schema::usuariosrust::apellidos = String::from("ramillll");
//     let self::schema::usuariosrust::id: i32 = 3;

//     let nuevo_usuario = Usuariosrust {
//         id,
//         nombre,
//         apellidos,
//     };

//     diesel::insert_into(usuariosrust::table)
//         .values(nuevo_usuario)
//         .execute(conn)
// }



pub fn crear_usuario(){
    let connection = &mut establish_connection();
    let nombre_info = String::from("titoooooinsertadoooo");
    let apellidos_info = String::from("titooooo");
    use self::schema::usuariosrust::dsl::*;


   

    let nuevo_usuario:NuevoUsuario = NuevoUsuario {nombre:nombre_info, apellidos:apellidos_info };





    let usuario_creado=diesel::insert_into(usuariosrust)
        .values(nuevo_usuario)
        .execute(connection)
        .expect("Error creando el usuario");
 
}
