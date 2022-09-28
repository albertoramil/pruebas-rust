

use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct USuariosrust {
    pub  id :i32,
    pub  nombre :String,
    pub  apellidos :String,
}
