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



pub fn actualizar_usuario(id_usuario:i32){
    use self::schema::usuariosrust::dsl::*;
    let connection = &mut establish_connection();
    let usuario_actualizado=diesel::update(usuariosrust.filter(id.eq(id_usuario)))
    .set((nombre.eq("James"), apellidos.eq("Not Bond")))
    .execute(connection).unwrap();
    if usuario_actualizado==1{
        println!("Usuario con id {} actualizado correctamente!",id_usuario)
    }else{
        println!("Se ha producido un error al actualizar el usuario con id {}",id_usuario)
    }
}
