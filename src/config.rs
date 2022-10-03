extern crate dotenv;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub url_odoo: String,
    pub db_odoo: String,
    pub username_odoo: String,
    pub password_odoo: String,
}
//Variables de entorno
pub fn get_config() -> Config {
    dotenv().ok();
    Config {
        url_odoo: std::env::var("ODOO_URL")
            .expect("No se pudo acceder al valor de la variable de entorno ODOO_URL"),
        db_odoo: std::env::var("ODOO_DB")
            .expect("No se pudo acceder al valor de la variable de entorno ODOO_DB"),
        username_odoo: std::env::var("ODOO_USERNAME")
            .expect("No se pudo acceder al valor de la variable de entorno ODOO_USERNAME"),
        password_odoo: std::env::var("ODOO_PASSWORD")
            .expect("No se pudo acceder al valor de la variable de entorno ODOO_PASSWORD"),
    }
}
