



extern crate rust_comienzo;
extern crate diesel;

use self::rust_comienzo::*;
use self::models::*;
use self::diesel::prelude::*;




pub fn get_usuarios(){
    let connection = &mut establish_connection();
    use self::schema::usuariosrust::dsl::*;
    let results = usuariosrust       
        .limit(5)
        .load::<USuariosrust>(connection)
        .expect("Error obteniendo los usuarios");


        for usuario in results{
           
            println!("Nombre usuario: {}, apellidos: {} ",usuario.nombre,usuario.apellidos);
        }
    
}
