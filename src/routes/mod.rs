pub mod accounts;
pub mod reader;

#[catch(403)]
pub fn unauthorized() -> &'static str {
    "Unauthorized"
}
