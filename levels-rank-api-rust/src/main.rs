#[macro_use]
extern crate rocket;
extern crate rocket_cors;

mod controllers;
pub mod cors;
pub mod db;
pub mod helpers;
pub mod models;

use controllers::players_controller;
use dotenv::dotenv;
use rocket::{http::Status, Request};

use serde_json::{json, Value};
use std::sync::Arc;

pub type Ctx = rocket::State<Context>;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<db::PrismaClient>,
}

#[catch(default)]
fn default(status: Status, req: &Request) -> Value {
    json!({ "status": status.code, "url": req.uri(), })
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    // prisma client
    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .attach(cors::CORS)
        .manage(Context { db })
        .register("/", catchers![default])
        .mount("/", routes![teste])
        .mount("/players", players_controller::routes())
}

#[get("/teste")]
fn teste() -> String {
    String::from("teste")
}
