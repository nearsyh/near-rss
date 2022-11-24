pub mod edit;
pub mod stream;
pub mod subscriptions;
pub mod users;

use crate::middlewares::auth::AuthUser;
use actix_web::{web, HttpResponse};

fn convert_to_long_form_ids(ids: &Vec<&str>) -> Vec<String> {
    ids.iter()
        .map(|id| format!("tag:google.com,2005:reader/item/{}", id))
        .collect::<Vec<String>>()
}

pub async fn ping(_token: web::ReqData<AuthUser>) -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
