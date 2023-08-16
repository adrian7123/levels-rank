use crate::Ctx;

use crate::models::player::{PlayerDto, Steam};
use db::lvl_base;
use discord_bot::bot_helper::BotHelper;
use prisma_client_rust::Direction;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use serde::Deserialize;
use serde::Serialize;
use serenity::model::prelude::ChannelId;
use serenity::utils::MessageBuilder;
use shared::steam_helper::SteamHelper;
use std::env;

pub fn routes() -> Vec<Route> {
    routes![get_players, post_logs, post_end_map]
}

#[get("/")]
async fn get_players(ctx: &Ctx) -> Json<Vec<PlayerDto>> {
    let db_players: Vec<lvl_base::Data> = ctx
        .db
        .lvl_base()
        .find_many(vec![])
        .order_by(lvl_base::value::order(Direction::Desc))
        .exec()
        .await
        .unwrap();

    let steam_ids: Vec<String> = db_players
        .iter()
        .map(|player| player.steam.clone())
        .collect();

    let steam_players: Vec<Steam> = SteamHelper::get_all_steam_players(steam_ids).await;

    let mut id: u16 = 0;

    let players: Vec<PlayerDto> = db_players
        .iter()
        .map(|player| {
            id += 1;
            let steam: &Steam = steam_players
                .iter()
                .find(|sp: &&Steam| {
                    sp.steamid == Some(SteamHelper::convert_steam_id(player.steam.clone()))
                })
                .unwrap();

            let mut player_dto: PlayerDto =
                serde_json::from_value(serde_json::to_value(&player).unwrap()).unwrap();

            player_dto.set_steam_data(steam.clone());
            player_dto.set_id(id);
            player_dto
        })
        .collect();

    Json(players)
}

#[post("/end_map")]
async fn post_end_map(_ctx: &Ctx) {
    ()
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct LogsBody {
    admin_name: String,
    player_name: String,
    value: String,
}

#[post("/logs", data = "<body>")]
async fn post_logs(_ctx: &Ctx, body: Json<LogsBody>) -> status::Custom<Json<LogsBody>> {
    let http = _ctx.discord_client.cache_and_http.http.clone();

    let bot_helper = BotHelper::new(http.clone());

    let channel: ChannelId = ChannelId(
        env::var("DISCORD_LOG_CHANNEL")
            .expect("")
            .parse::<u64>()
            .expect(""),
    );

    let mut message_builder: MessageBuilder = MessageBuilder::new()
        .push_bold(&body.admin_name)
        .push(format!(
            " {} ",
            if body.value.contains("+") {
                "adicionou"
            } else {
                "removeu"
            }
        ))
        .push_bold(&body.value)
        .push(" para ")
        .push_bold(&body.player_name)
        .clone();

    let _ = bot_helper
        .send_message(channel, &http, &mut message_builder)
        .await;

    status::Custom(Status::Accepted, body)
}
