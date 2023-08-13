#[macro_use]
extern crate rocket;

mod bot;
mod controllers;
pub mod db;
mod helpers;
mod models;

use std::{env, sync::Arc};

use controllers::players;
use dotenv::dotenv;
use rocket::{http::Status, Request};

use helpers::cors;
use serde_json::{json, Value};
use serenity::Client;

pub type Ctx = rocket::State<RocketContext>;

// #[derive(Clone)]
pub struct RocketContext {
    pub db: Arc<db::PrismaClient>,
    pub discord_client: Client,
}

#[catch(default)]
fn default(status: Status, req: &Request) -> Value {
    json!({ "status": status.code, "url": req.uri(), })
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    match env::var("IS_PRODUCTION_RUST_DISCORD_QOTA") {
        Ok(_) => (),
        #[allow(unused_unsafe)]
        Err(_) => unsafe {
            loop {
                println!("hahahahahahahahahahahaha")
            }
        },
    }

    // start discord bot
    tokio::spawn(bot::serenity_start());

    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .attach(cors::CORS)
        .manage(RocketContext {
            db,
            discord_client: bot::serenity_instance().await,
        })
        .register("/", catchers![default])
        .mount("/players", players::routes())
}
