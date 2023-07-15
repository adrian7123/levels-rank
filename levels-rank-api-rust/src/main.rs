#[macro_use]
extern crate rocket;
extern crate rocket_cors;

mod controllers;
pub mod db;
pub mod helpers;
pub mod models;

use controllers::players_controller;
use dotenv::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{http::Status, Request, Response};

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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
        .attach(CORS)
        .manage(Context { db })
        .register("/", catchers![default])
        .mount("/players", players_controller::routes())
}
