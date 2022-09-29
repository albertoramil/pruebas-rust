

use crate::schema::*;
use diesel::prelude::*;
#[derive(Debug)]
#[derive(Queryable,AsChangeset)]
#[diesel(table_name = usuariosrust)]
pub struct Usuariosrust {
    pub  id :i32,
    pub  nombre :String,
    pub  apellidos :String,
}





#[derive(Insertable,AsChangeset)]
#[diesel(table_name = usuariosrust)]
pub struct NuevoUsuario {
    pub nombre:  String,
    pub apellidos:  String,
}
