#[macro_use]
extern crate rocket;

use near_rss::create;

#[launch]
async fn rocket() -> _ {
    create().await
}
