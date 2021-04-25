pub mod subscriptions;
pub mod users;

use crate::middlewares::auth::AuthToken;

#[get("/ping")]
pub fn ping(_token: AuthToken) -> &'static str {
    "OK"
}