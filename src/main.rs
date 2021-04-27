#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod common;
mod database;
mod middlewares;
mod routes;
mod services;

async fn init() {
    let user = services::users::new_user_service()
        .await
        .register("nearsy.h@gmail.com", "1234")
        .await
        .unwrap();

    services::subscriptions::new_subscription_service()
        .await
        .add_subscription_from_url(&user.id, "https://www.daemonology.net/hn-daily/index.rss")
        .await
        .unwrap();
}

#[launch]
async fn rocket() -> _ {
    init().await;
    rocket::build()
        .mount("/accounts", routes![routes::accounts::client_login])
        .mount(
            "/reader",
            routes![
                routes::reader::ping,
                routes::reader::subscriptions::list_subscriptions,
                routes::reader::users::get_user_info,
                routes::reader::stream::get_item_ids,
                routes::reader::stream::get_contents,
            ],
        )
        .register("/", catchers![routes::unauthorized])
}
