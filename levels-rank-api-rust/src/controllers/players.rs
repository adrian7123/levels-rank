use crate::Ctx;

use crate::bot::bot_helper::BotHelper;
use crate::db::lvl_base;
use crate::helpers::steam::convert_steam_id;
use crate::helpers::steam::get_all_steam_players;
use crate::models::player::{PlayerDto, Steam};
use prisma_client_rust::Direction;
use rocket::http::Status;
use rocket::log::private::info;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Route;
use serde::Deserialize;
use serde::Serialize;
use serenity::model::prelude::ChannelId;
use serenity::utils::MessageBuilder;
use std::env;
use tabled::settings::Style;
use tabled::{Table, Tabled};

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

    let steam_players: Vec<Steam> = get_all_steam_players(steam_ids).await;

    let mut id: u16 = 0;

    let players: Vec<PlayerDto> = db_players
        .iter()
        .map(|player| {
            id += 1;
            let steam: &Steam = steam_players
                .iter()
                .find(|sp: &&Steam| sp.steamid == Some(convert_steam_id(player.steam.clone())))
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct LogsBody {
    admin_name: String,
    player_name: String,
    value: String,
}

#[derive(Tabled)]
struct Language {
    name: String,
    designed_by: String,
    invented_year: usize,
}

impl Language {
    fn new(name: &str, designed_by: &str, invented_year: usize) -> Self {
        Self {
            name: name.to_string(),
            designed_by: designed_by.to_string(),
            invented_year,
        }
    }
}

#[post("/end_map")]
async fn post_end_map(_ctx: &Ctx) {
    let http = _ctx
        .discord_client
        .as_ref()
        .unwrap()
        .cache_and_http
        .http
        .clone();

    let bot_helper = BotHelper::new(http.clone());

    let languages = vec![
        Language::new("C", "Dennis Ritchie", 1972),
        Language::new("Go", "Rob Pike", 2009),
        Language::new("Rust", "Graydon Hoare", 2010),
        Language::new("Hare", "Drew DeVault", 2022),
    ];

    let table = Table::new(languages).with(Style::sharp()).to_string();

    let mut message_builder: MessageBuilder = MessageBuilder::new()
        .push_line(format!("```{}```", &table))
        .clone();

    let channel: ChannelId = ChannelId(
        env::var("DISCORD_CHANNEL")
            .expect("")
            .parse::<u64>()
            .expect(""),
    );

    let _ = bot_helper
        .send_message(channel, &http, &mut message_builder)
        .await;

    info!("{:?}", table);
}

#[post("/logs", data = "<body>")]
async fn post_logs(_ctx: &Ctx, body: Json<LogsBody>) -> status::Custom<Json<LogsBody>> {
    let http = _ctx
        .discord_client
        .as_ref()
        .unwrap()
        .cache_and_http
        .http
        .clone();

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
