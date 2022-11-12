#[macro_use]
extern crate rocket;

use near_rss::configuration::get_configuration;
use near_rss::create;

#[launch]
async fn rocket() -> _ {
    let configuration = get_configuration().expect("Failed to get configuration.");
    let app = create(&configuration).await;
    app.rocket
}
