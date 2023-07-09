#[macro_use]
extern crate rocket;

mod app;
pub mod db;
pub mod helpers;
pub mod models;

use app::steam_controller::get_all_steam_players;
use db::lvl_base;
use dotenv::dotenv;
use helpers::steam_helper::convert_steam_id;
use models::player::{PlayerDto, Steam};
use rocket::serde::json::Json;
use std::sync::Arc;

pub type Ctx = rocket::State<Context>;

#[get("/players")]
async fn get_players(ctx: &Ctx) -> Json<Vec<PlayerDto>> {
    let db_players: Vec<lvl_base::Data> = ctx.db.lvl_base().find_many(vec![]).exec().await.unwrap();

    let steam_ids: Vec<String> = db_players
        .iter()
        .map(|player| player.steam.clone())
        .collect();

    let steam_players: Vec<Steam> = get_all_steam_players(steam_ids).await;

    let players: Vec<PlayerDto> = db_players
        .iter()
        .map(|player| {
            let steam: &Steam = steam_players
                .iter()
                .find(|sp: &&Steam| sp.steamid == Some(convert_steam_id(player.steam.clone())))
                .unwrap();

            let mut player_dto: PlayerDto =
                serde_json::from_value(serde_json::to_value(&player).unwrap()).unwrap();

            player_dto.set_steam_data(steam.clone());
            player_dto
        })
        .collect();

    Json(players)
}

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
        .mount("/", routes![get_players])
}
