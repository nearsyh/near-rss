pub mod subscriptions;
pub mod users;
pub mod stream;

use crate::middlewares::auth::AuthToken;

#[get("/ping")]
pub fn ping(_token: AuthToken) -> &'static str {
    "OK"
}
