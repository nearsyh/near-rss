pub mod subscriptions;

use crate::middlewares::auth::AuthToken;

#[get("/ping")]
pub fn ping(_token: AuthToken) -> &'static str {
    "OK"
}