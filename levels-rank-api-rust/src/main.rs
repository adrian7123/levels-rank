#[macro_use]
extern crate rocket;

mod helpers;

#[allow(warnings, unused)]
pub mod db;

use std::sync::Arc;

use db::lvl_base;
use rocket::serde::json::Json;

#[derive(Clone)]
pub struct Context {
    pub db: Arc<db::PrismaClient>,
}

pub type Ctx = rocket::State<Context>;

#[get("/players")]
async fn index(ctx: &Ctx) -> Json<Vec<lvl_base::Data>> {
    let players: Vec<lvl_base::Data> = ctx.db.lvl_base().find_many(vec![]).exec().await.unwrap();

    println!(
        "{:?}",
        helpers::steam::convert_steam_id(players[0].steam.clone())
    );

    test();

    Json(players)
}

#[tokio::main]
async fn test() {
    let result: Result<reqwest::Response, reqwest::Error> =
        reqwest::get("https://api.spotify.com/v1/search").await;

    println!("{:?}", result)
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        db::new_client()
            .await
            .expect("Failed to create Prisma client"),
    );

    #[cfg(debug_assert)]
    db._db_push(false).await.unwrap();

    rocket::build()
        .manage(Context { db })
        .mount("/", routes![index])
}
