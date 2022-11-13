#[macro_use]
extern crate rocket;

use near_rss::configuration::get_configuration;
use near_rss::{Application, ServerWrapper};

#[launch]
async fn rocket() -> _ {
    let configuration = get_configuration().expect("Failed to get configuration.");
    let app = Application::create_rocket_server(&configuration)
        .await
        .expect("Failed to create application");
    match app.server {
        ServerWrapper::RocketServer(rocket) => rocket,
        _ => panic!("Not supported type"),
    }
}
