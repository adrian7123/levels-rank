use std::env;

use serde_json::Value;

pub struct SteamHelper;

impl SteamHelper {
    pub async fn get_all_steam_players<T>(steam_ids: Vec<String>) -> Vec<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut string_steam_players: String = String::new();

        let client = reqwest::Client::new();

        let steam_web_key = env::var("STEAM_WEB_KEY").expect("DB_HOST não definido");

        let converted_steam_ids = steam_ids
            .iter()
            .map(|n| SteamHelper::convert_steam_id(n.to_string()))
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

        let steam_players_value: Value =
            serde_json::from_str(string_steam_players.as_str()).unwrap();

        let steam_players: Vec<T> =
            serde_json::from_value(steam_players_value["response"]["players"].clone()).unwrap();

        steam_players
    }

    pub fn convert_steam_id(steam_id: String) -> String {
        if steam_id.starts_with("STEAM_") {
            let parts: Vec<&str> = steam_id.split(':').collect();
            let result: u64 = parts[2].parse::<u64>().unwrap() * 2
                + 76561197960265728
                + parts[1].parse::<u64>().unwrap();
            return result.to_string();
        } else if let Ok(id) = steam_id.parse::<u64>() {
            if steam_id.len() < 16 {
                let result: u64 = id + 76561197960265728;
                return result.to_string();
            }
        }
        steam_id.to_string()
    }
}
