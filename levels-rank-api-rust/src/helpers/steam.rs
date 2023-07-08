pub fn convert_steam_id(steam_id: String) -> String {
    if steam_id.starts_with("STEAM_") {
        let parts: Vec<&str> = steam_id.split(':').collect();
        let result = parts[2].parse::<u64>().unwrap() * 2
            + 76561197960265728
            + parts[1].parse::<u64>().unwrap();
        return result.to_string();
    } else if let Ok(id) = steam_id.parse::<u64>() {
        if steam_id.len() < 16 {
            let result = id + 76561197960265728;
            return result.to_string();
        }
    }
    steam_id.to_string()
}
