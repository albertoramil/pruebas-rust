

use crate::schema::*;
use diesel::prelude::*;
#[derive(Queryable,Insertable,Debug)]
#[diesel(table_name = usuariosrust)]
pub struct Usuariosrust {
    pub  id :i32,
    pub  nombre :String,
    pub  apellidos :String,
}
