#[macro_use]
extern crate rocket;

use near_rss::create;
use near_rss::configuration::get_configuration;

#[launch]
async fn rocket() -> _ {
    let configuration = get_configuration().expect("Failed to get configuration.");
    create(&configuration).await
}
