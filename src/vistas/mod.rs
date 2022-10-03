use crate::odoo::{get_odoo, PartnerTemplate};
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub async fn get_partners() -> Response {
    //Conexion odoo
    let odoo = get_odoo().await;
    
    // Consulta
    let listado: Vec<PartnerTemplate> = odoo
        .search_read(
            "res.partner",
            // (("name", "=", "Norvoz Telecom, S.L."), ("id", "<", "9")),
            (("id", "<", "9"),),
            // (),
            Some(vec!["id", "name", "vat"]),
            None,
            None,
        )
        .await.unwrap().result;

    let partners = ListadoPartners{
        partners: listado,
    };

    Json(partners).into_response()
}

#[derive(Serialize)]
struct ListadoPartners {
    partners: Vec<PartnerTemplate>,
}
