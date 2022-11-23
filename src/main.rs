#[macro_use]
extern crate rocket;

use near_rss::configuration::get_configuration;
use near_rss::refresh::refresh_until_stopped;
use near_rss::{Application, ServerWrapper};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

// #[launch]
// async fn rocket() -> _ {
//     let configuration = get_configuration().expect("Failed to get configuration.");
//     let app = Application::create_rocket_server(&configuration)
//         .await
//         .expect("Failed to create application");
//     match app.server {
//         ServerWrapper::RocketServer(rocket) => rocket,
//         _ => panic!("Not supported type"),
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    // init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::create_actix_server(&configuration).await?;

    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(refresh_until_stopped(configuration));
    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background worker", o),
    }
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name )
        }
    }
}
