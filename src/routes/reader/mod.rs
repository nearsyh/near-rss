pub mod stream;
pub mod subscriptions;
pub mod users;

use crate::middlewares::auth::AuthUser;

#[get("/ping")]
pub fn ping(_token: AuthUser) -> &'static str {
    "OK"
}
