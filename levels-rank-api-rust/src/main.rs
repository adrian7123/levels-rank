#[macro_use]
extern crate rocket;

mod controllers;
pub mod db;
pub mod helpers;
pub mod models;

use controllers::players_controller;
use dotenv::dotenv;
use rocket::{http::Method, http::Status, Request};
use rocket_cors::{AllowedOrigins, CorsOptions};
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

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        // Other CORS options like allowed_methods, allowed_headers, etc.
        // can be configured here.
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS middleware.");

    // prisma client
    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .attach(cors)
        .manage(Context { db })
        .register("/", catchers![default])
        .mount("/players", players_controller::routes())
}
