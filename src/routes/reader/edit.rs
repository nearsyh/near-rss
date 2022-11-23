use crate::common::Services;
use crate::middlewares::auth::AuthUser;
use actix_web::{web, HttpResponse};
use rocket::form::Form;

#[derive(FromForm)]
pub struct OldEditTagRequest<'r> {
    pub i: Option<Vec<&'r str>>,
    pub a: Option<&'r str>,
    pub r: Option<&'r str>,
}

#[post("/api/0/edit-tag", data = "<request>")]
pub async fn old_edit_tag(
    auth_user: AuthUser,
    services: &Services,
    request: Form<OldEditTagRequest<'_>>,
) -> &'static str {
    let user_id = &auth_user.user.id;
    if let Some(ref ids) = request.i {
        let ids_in_hex = super::convert_to_long_form_ids(ids);
        let ids_ref = &ids_in_hex.iter().map(|s| &**s).collect::<Vec<&str>>();
        if let Some(to_add) = request.a {
            if to_add == "user/-/state/com.google/read" {
                services
                    .stream_service
                    .mark_as_read(user_id, ids_ref)
                    .await
                    .unwrap();
            } else if to_add == "user/-/state/com.google/starred" {
                services
                    .stream_service
                    .mark_as_starred(user_id, ids_ref)
                    .await
                    .unwrap();
            }
        } else if let Some(to_remove) = request.r {
            if to_remove == "user/-/state/com.google/read" {
                services
                    .stream_service
                    .mark_as_unread(user_id, ids_ref)
                    .await
                    .unwrap();
            } else if to_remove == "user/-/state/com.google/starred" {
                services
                    .stream_service
                    .mark_as_unstarred(user_id, ids_ref)
                    .await
                    .unwrap();
            }
        }
    }
    "OK"
}

#[derive(FromForm)]
pub struct EditTagRequest {
    pub i: Option<Vec<String>>,
    pub a: Option<String>,
    pub r: Option<String>,
}

pub async fn edit_tag(
    auth_user: web::ReqData<AuthUser>,
    services: web::Data<Services>,
    request: web::Form<EditTagRequest>,
) -> HttpResponse {
    let user_id = &auth_user.user.id;
    if let Some(ref ids) = request.i {
        let ids_in_hex = super::convert_to_long_form_ids(ids);
        let ids_ref = &ids_in_hex.iter().map(|s| &**s).collect::<Vec<&str>>();
        if let Some(to_add) = &request.a {
            if to_add == "user/-/state/com.google/read" {
                services
                    .stream_service
                    .mark_as_read(user_id, ids_ref)
                    .await
                    .unwrap();
            } else if to_add == "user/-/state/com.google/starred" {
                services
                    .stream_service
                    .mark_as_starred(user_id, ids_ref)
                    .await
                    .unwrap();
            }
        } else if let Some(to_remove) = &request.r {
            if to_remove == "user/-/state/com.google/read" {
                services
                    .stream_service
                    .mark_as_unread(user_id, ids_ref)
                    .await
                    .unwrap();
            } else if to_remove == "user/-/state/com.google/starred" {
                services
                    .stream_service
                    .mark_as_unstarred(user_id, ids_ref)
                    .await
                    .unwrap();
            }
        }
    }
    HttpResponse::Ok().body("OK")
}
