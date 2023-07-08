use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub name: &'static str,
    pub age: u8,
}
