use std::env;

use crate::db::{self, mix};
use prisma_client_rust::Direction;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{ChannelId, Guild, Member, Message};
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use super::helpers;

const MIN_PLAYERS: u8 = 10;

#[group]
#[commands(ping, times, entrar)]
pub struct General;

#[command]
async fn entrar(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");
    let last_table = db
        .mix()
        .find_many(vec![])
        .order_by(mix::created_at::order(Direction::Asc))
        .exec()
        .await
        .unwrap();

    if last_table.len() <= 0 {
        let _ = db.mix().create(vec![]).exec().await;
    }

    let _ = msg.reply(ctx, "(beta) criado com sucesso").await;

    Ok(())
}

#[command]
async fn times(ctx: &Context, msg: &Message) -> CommandResult {
    let channel_ruffle_id = env::var("DISCORD_RUFFLE_CHANNEL")
        .expect("err DISCORD_LOG_CHANNEL")
        .parse::<u64>()
        .expect("err parse discord channel id");

    let guild: Guild = msg.guild_id.expect("err").to_guild_cached(ctx).unwrap();

    let channel: ChannelId = ChannelId(channel_ruffle_id);

    // let _ = channel.broadcast_typing(ctx).await;

    let members_in_channel: Vec<Member> = helpers::members_in_channel(guild, channel);

    for member in &members_in_channel {
        println!(
            "Member ID: {}, Member Name: {} {}",
            member.user.id,
            member.user.name,
            &members_in_channel.len()
        );
    }

    if members_in_channel.len() <= 10 {
        let _ = msg
            .reply(
                &ctx.http,
                MessageBuilder::new()
                    .push("Necessário ")
                    .push_bold(MIN_PLAYERS)
                    .push(" membros, ")
                    .push("na call ")
                    .mention(&channel.to_channel(ctx).await.unwrap())
                    .push(" para o sorteio. ")
                    .push("\nFaltam ")
                    .push_bold(MIN_PLAYERS - members_in_channel.len() as u8)
                    .build(),
            )
            .await;

        return Ok(());
    }

    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = msg
        .channel_id
        .to_channel(&ctx)
        .await
        .expect("Error getting channel");

    // The message builder allows for creating a message by
    // mentioning users dynamically, pushing "safe" versions of
    // content (such as bolding normalized content), displaying
    // emojis, and more.
    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push(" used the 'ping' command in the ")
        .mention(&channel)
        .push(" channel")
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
