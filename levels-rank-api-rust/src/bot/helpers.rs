use serenity::{
    http::Http,
    model::prelude::{ChannelId, Guild, Member, Message},
    prelude::SerenityError,
    utils::MessageBuilder,
};

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
