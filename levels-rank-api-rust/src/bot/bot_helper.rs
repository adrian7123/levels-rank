use serenity::{
    http::{CacheHttp, Http},
    model::prelude::{ChannelId, Guild, GuildId, Member, Message, RoleId, UserId},
    prelude::SerenityError,
    utils::MessageBuilder,
};

pub struct BotHelper {
    cache_http: Box<dyn CacheHttp>,
}

#[allow(dead_code)]
impl BotHelper {
    pub fn new(cache_http: impl CacheHttp + 'static) -> Self {
        Self {
            cache_http: Box::new(cache_http),
        }
    }
    pub async fn add_member_role(&self, guild_id: GuildId, user_id: UserId, role_id: String) {
        let _ = guild_id
            .member(self.cache_http.http(), user_id)
            .await
            .expect("asd")
            .add_role(
                self.cache_http.http(),
                RoleId(role_id.parse::<u64>().expect("err")),
            )
            .await;
    }
    pub async fn remove_member_role(&self, guild_id: GuildId, user_id: UserId, role_id: String) {
        let _ = guild_id
            .member(self.cache_http.http(), user_id)
            .await
            .expect("Error remove_member_role")
            .remove_role(
                self.cache_http.http(),
                RoleId(role_id.parse::<u64>().expect("err")),
            )
            .await;
    }
    pub fn parse_mention(&self, mention: String) -> Result<u64, std::num::ParseIntError> {
        mention.replace("<@", "").replace(">", "").parse::<u64>()
    }
    pub fn members_in_channel(&self, guild: Guild, channel_id: ChannelId) -> Vec<Member> {
        // Obtenha os membros no canal de voz específico.
        let voice_states = guild.voice_states;

        // Filtrar os membros que estão no canal de voz específico.
        let members_in_channel: Vec<Member> = voice_states
            .into_iter()
            .filter_map(|(_, voice_state)| {
                if let Some(channel) = voice_state.channel_id {
                    if channel == channel_id {
                        return guild.members.get(&voice_state.user_id).cloned();
                    }
                }
                None
            })
            .collect();

        members_in_channel
    }

    pub async fn send_message(
        &self,
        channel: ChannelId,
        http: &Http,
        message_builder: &mut MessageBuilder,
    ) -> Result<Message, SerenityError> {
        let response = message_builder.build();

        channel.say(http, &response).await
    }
    pub async fn get_members_by_ids(&self, guild_id: GuildId, ids: Vec<String>) -> Vec<Member> {
        let mut members: Vec<Member> = vec![];
        for id in ids {
            members.push(
                guild_id
                    .member(
                        self.cache_http.http(),
                        UserId::from(id.parse::<u64>().expect("id.parse() err")),
                    )
                    .await
                    .expect("err get_member"),
            );
        }
        members
    }
    pub async fn get_member(
        &self,
        guild_id: GuildId,
        cache_http: impl CacheHttp,
        user_id: impl Into<UserId>,
    ) -> Member {
        guild_id
            .member(cache_http, user_id)
            .await
            .expect("err get_member")
    }
}
