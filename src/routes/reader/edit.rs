use crate::common::Services;
use crate::middlewares::auth::AuthUser;
use rocket::form::Form;

#[derive(FromForm)]
pub struct EditTagRequest<'r> {
    pub i: Option<Vec<&'r str>>,
    pub a: Option<&'r str>,
    pub r: Option<&'r str>,
}

#[post("/api/0/edit-tag", data = "<request>")]
pub async fn edit_tag(
    auth_user: AuthUser,
    services: &Services,
    request: Form<EditTagRequest<'_>>,
) -> &'static str {
    let user_id = &auth_user.user.id;
    if let Some(ref ids) = request.i {
        if let Some(to_add) = request.a {
            if to_add == "user/-/state/com.google/read" {
                services.stream_service.mark_as_read(user_id, ids).await.unwrap();
            } else if to_add == "user/-/state/com.google/starred" {
                services.stream_service.mark_as_starred(user_id, ids).await.unwrap();
            }
        } else if let Some(to_remove) = request.r {
            if to_remove == "user/-/state/com.google/read" {
                services.stream_service.mark_as_unread(user_id, ids).await.unwrap();
            } else if to_remove == "user/-/state/com.google/starred" {
                services.stream_service.mark_as_unstarred(user_id, ids).await.unwrap();
            }
        }
    }
    "OK"
}