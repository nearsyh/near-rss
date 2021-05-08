pub mod accounts;
pub mod reader;
pub mod ui;

#[catch(403)]
pub fn unauthorized() -> &'static str {
    "Unauthorized"
}
