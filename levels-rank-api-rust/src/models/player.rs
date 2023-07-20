use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerDto {
    pub id: Option<u16>,
    pub steam: Option<String>,
    pub steam_data: Option<Steam>,
    pub name: Option<String>,
    pub value: Option<u32>,
    pub rank: Option<u32>,
    pub kills: Option<u32>,
    pub deaths: Option<u32>,
    pub shoots: Option<u32>,
    pub hits: Option<u32>,
    pub headshots: Option<u32>,
    pub assists: Option<u32>,
    pub round_win: Option<u32>,
    pub round_lose: Option<u32>,
    pub playtime: Option<u32>,
    pub lastconnect: Option<u64>,
}

impl PlayerDto {
    pub fn set_steam_data(&mut self, steam: Steam) {
        self.steam_data = Some(steam);
    }
    pub fn set_id(&mut self, id: u16) {
        self.id = Some(id);
    }
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self {
            id: None,
            steam: None,
            steam_data: None,
            name: None,
            value: None,
            rank: None,
            kills: None,
            deaths: None,
            shoots: None,
            hits: None,
            headshots: None,
            assists: None,
            round_win: None,
            round_lose: None,
            playtime: None,
            lastconnect: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Steam {
    pub steamid: Option<String>,
    communityvisibilitystate: Option<u8>,
    profilestate: Option<u8>,
    personaname: Option<String>,
    commentpermission: Option<u8>,
    profileurl: Option<String>,
    avatar: Option<String>,
    avatarmedium: Option<String>,
    avatarfull: Option<String>,
    avatarhash: Option<String>,
    lastlogoff: Option<u64>,
    personastate: Option<u8>,
    primaryclanid: Option<String>,
    timecreated: Option<u64>,
    personastateflags: Option<u8>,
    loccountrycode: Option<String>,
}
