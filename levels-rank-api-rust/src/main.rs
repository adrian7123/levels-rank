#[macro_use]
extern crate rocket;

mod controllers;
mod models;

use std::{env, sync::Arc};

use controllers::players;
use dotenv::dotenv;
use rocket::{http::Status, Request};

use serde_json::{json, Value};
use serenity::Client;
mod cors;

pub type Ctx = rocket::State<RocketContext>;

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

    let token: String = env::var("DISCORD_TOKEN").expect("token");

    // ! gere seu próprio binário
    match env::var("IS_PRODUCTION_RUST_DISCORD_QOTA") {
        Ok(_) => (),
        #[allow(unused_unsafe)]
        Err(_) => unsafe {
            loop {
                println!("hahahahahahahahahahahaha")
            }
        },
    }

    // ? start discord bot 2
    tokio::spawn(discord_bot::serenity_start(token.clone()));

    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    // teste
    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .attach(cors::CORS)
        .manage(RocketContext {
            db,
            discord_client: discord_bot::serenity_instance(token).await,
        })
        .register("/", catchers![default])
        .mount("/players", players::routes())
}
