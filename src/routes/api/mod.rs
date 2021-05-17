use crate::common::{PageOption, Services};
use crate::middlewares::auth::AuthUser;
use crate::services::stream::ItemContent;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    items: Vec<ItemContent>,
    next_page_offset: Option<String>,
}

#[get("/unread?<offset>&<limit>")]
pub async fn get_unread_items(
    auth_user: AuthUser,
    services: &Services,
    offset: Option<String>,
    limit: Option<usize>,
) -> Json<Contents> {
    let user_id = &auth_user.user.id;
    let contents = services.stream_service.get_unread_item_contents(user_id, PageOption {
        offset: offset,
        limit: limit.unwrap_or(100),
        desc: true,
    }).await.unwrap();
    Json(Contents {
        items: contents.items,
        next_page_offset: contents.next_page_offset,
    })
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ids {
    ids: Vec<String>,
}

#[post("/markAsRead", data = "<ids>")]
pub async fn mark_as_read(auth_user: AuthUser, services: &Services, ids: Json<Ids>) -> &'static str {
    let str_ids = ids.ids.iter().map(|s| &**s).collect();
    services.stream_service.mark_as_read(&auth_user.user.id, &str_ids).await.unwrap();
    "OK"
}
