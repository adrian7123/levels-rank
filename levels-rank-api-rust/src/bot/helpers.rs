use serenity::{
    http::{CacheHttp, Http},
    model::prelude::{ChannelId, Guild, GuildId, Member, Message, UserId},
    prelude::SerenityError,
    utils::MessageBuilder,
};

use crate::db::{mix, mix_player};

pub fn members_in_channel(guild: Guild, channel_id: ChannelId) -> Vec<Member> {
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

pub async fn bot_send_message(
    channel: ChannelId,
    http: &Http,
    message_builder: &mut MessageBuilder,
) -> Result<Message, SerenityError> {
    let response = message_builder.build();

    channel.say(http, &response).await
}

pub fn bot_make_message_mix_list(mix: mix::Data, players: Vec<mix_player::Data>) -> MessageBuilder {
    let mut message: MessageBuilder = MessageBuilder::new();

    message
        .push("Mix Que Ota Community Hoje ")
        .push(mix.date.format("**%d/%m** "))
        .push(mix.date.format("**%H:%M** "))
        .push("\n\n");
    let mut pos: u8 = 0;
    for player in players {
        pos += 1;
        message.push_bold(format!("{}  -  <@{}>", pos, player.discord_id));
        message.push("\n");
    }

    message.push("\n");

    message
}

pub async fn bot_get_member(
    guild_id: GuildId,
    cache_http: impl CacheHttp,
    user_id: impl Into<UserId>,
) -> Member {
    guild_id
        .member(cache_http, user_id)
        .await
        .expect("err bot_get_member")
}
