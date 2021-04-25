use crate::middlewares::AuthToken;

#[get("/ping")]
pub fn ping(_token: AuthToken) -> &'static str {
    "OK"
}