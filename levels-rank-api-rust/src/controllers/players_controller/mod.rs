use crate::Ctx;

use crate::db::lvl_base;
use crate::helpers::steam_helper::convert_steam_id;
use crate::helpers::steam_helper::get_all_steam_players;
use crate::models::player::{PlayerDto, Steam};
use rocket::serde::json::Json;
use rocket::Route;

#[get("/")]
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

pub fn routes() -> Vec<Route> {
    routes![get_players]
}
