use crate::config::get_config;
use async_odoors::odoo::{deserialize_odoo_nullable, Odoo};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PartnerTemplate {
    pub id: u32,
    pub name: String,
    #[serde(deserialize_with = "deserialize_odoo_nullable")]
    pub vat: Option<String>,
}

pub async fn get_odoo() -> Odoo {
    let conf = get_config();
    Odoo::new_and_login(
        &conf.url_odoo,
        &conf.db_odoo,
        &conf.username_odoo,
        &conf.password_odoo,
    )
    .await.unwrap()
}
