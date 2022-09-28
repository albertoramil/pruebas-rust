
use reqwest;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Perfil {
     Rol: String,
     created_at: String,
     id: u32,
     updatedAt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    success: bool,
    tiposperfiles: Vec<Perfil>
}
pub fn imprimir(prod: &APIResponse) {
    println!("{:?}", prod);
   println!("Producto: {}", "prod.success");
   println!("Producto: {}", prod.success);
}


#[tokio::main]
pub async fn get() {
    let url = "http://localhost:3000/v1/tipoperfil/".to_string();
    let res = reqwest::get(url).await.unwrap();



    println!("pasandoporaquiii: {:?}", "3");
    match res.status() {
               reqwest::StatusCode::OK => {
                   // si foi ben parsease o json

                   match res.json::<APIResponse>().await {

                       Ok(parsed) => imprimir(&parsed),
                       Err(error) => println!("{:?}",error)
                   }
               }
               other => {
                   panic!("Explotou algo: {:?}", other);
               }
           }
}