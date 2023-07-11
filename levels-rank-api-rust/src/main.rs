#[macro_use]
extern crate rocket;

mod controllers;
pub mod db;
pub mod helpers;
pub mod models;

use controllers::players_controller;
use dotenv::dotenv;
use std::sync::Arc;

pub type Ctx = rocket::State<Context>;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<db::PrismaClient>,
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
        .manage(Context { db })
        .mount("/players", players_controller::routes())
}
