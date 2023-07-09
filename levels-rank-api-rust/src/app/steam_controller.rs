use std::env;

use serde_json::Value;

use crate::models::player::Steam;

use super::helpers::steam_helper::convert_steam_id;

pub async fn get_all_steam_players(steam_ids: Vec<String>) -> Vec<Steam> {
    let mut string_steam_players: String = String::new();

    let client = reqwest::Client::new();

    let steam_web_key = env::var("STEAM_WEB_KEY").expect("DB_HOST não definido");

    let converted_steam_ids = steam_ids
        .iter()
        .map(|n| convert_steam_id(n.to_string()))
        .collect::<Vec<String>>()
        .join(",");

    match client
        .get(format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={steam_web_key}&steamids={converted_steam_ids}",
        ))
        .send()
        .await
    {
        Ok(response) => {
            // Read the response body as a string
            string_steam_players = response.text().await.unwrap();

            // println!("{string_steam_players}")
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }

    let steam_players_value: Value = serde_json::from_str(string_steam_players.as_str()).unwrap();

    let steam_players: Vec<Steam> =
        serde_json::from_value(steam_players_value["response"]["players"].clone()).unwrap();

    steam_players
}
