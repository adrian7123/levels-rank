use std::env;

use super::helpers;
use crate::db::{self, mix, mix_player};
use chrono::Timelike;
use prisma_client_rust::Direction;
use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use serenity::model::prelude::{ChannelId, Guild, Member, Message, RoleId, UserId};
use serenity::model::Permissions;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

const MAX_PLAYERS: u8 = 10;

#[group]
#[commands(ping, times, entrar, comandos, sair, lista)]
pub struct General;

#[group]
#[commands(listadeespera, limparlista)]
#[checks(has_role)]
pub struct Admins;

#[check]
async fn has_role(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    if msg
        .member(&ctx)
        .await
        .unwrap()
        .roles(ctx)
        .unwrap()
        .iter()
        .any(|r| r.permissions.contains(Permissions::BAN_MEMBERS))
    {
        Ok(())
    } else {
        Err(Reason::Log(
            "You don't have the required role to use this command.".to_string(),
        ))
    }
}

#[command]
async fn lista(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");
    let mix = db
        .mix()
        .find_first(vec![mix::expired::equals(false)])
        .order_by(mix::created_at::order(Direction::Desc))
        .exec()
        .await
        .unwrap();

    if mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let players = db
        .mix_player()
        .find_many(vec![mix_player::mix_id::equals(Some(
            mix.clone().unwrap().id,
        ))])
        .order_by(mix_player::created_at::order(Direction::Asc))
        .exec()
        .await
        .unwrap();

    let mut players_table = helpers::bot_make_message_mix_list(mix.unwrap(), players.clone());

    let _ = msg.reply(ctx, players_table.build()).await;

    Ok(())
}

#[command]
async fn limparlista(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");
    let mix = db
        .mix()
        .find_first(vec![mix::expired::equals(false)])
        .order_by(mix::created_at::order(Direction::Desc))
        .exec()
        .await
        .unwrap();

    if mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let mut players = db
        .mix_player()
        .find_many(vec![mix_player::mix_id::equals(Some(
            mix.clone().unwrap().id,
        ))])
        .exec()
        .await
        .unwrap();

    // tira o cargo de todos os players da lista
    for player in players.clone() {
        let _ = helpers::bot_get_member(
            msg.guild_id.unwrap(),
            ctx,
            UserId::from(
                player
                    .discord_id
                    .parse::<u64>()
                    .expect("Invalid Discord ID"),
            ),
        )
        .await
        .remove_role(
            ctx,
            RoleId(
                env::var("DISCORD_LIST_CARGO_U64")
                    .expect("err")
                    .parse::<u64>()
                    .expect("err"),
            ),
        )
        .await;
    }

    // remover os players da lista
    let _ = db
        .mix_player()
        .delete_many(vec![mix_player::mix_id::equals(Some(
            mix.clone().unwrap().id,
        ))])
        .exec()
        .await
        .unwrap();

    players = vec![];

    let mut players_table = helpers::bot_make_message_mix_list(mix.unwrap(), players.clone());

    let _ = msg.reply(ctx, players_table.build()).await;

    Ok(())
}

#[command]
async fn comandos(ctx: &Context, msg: &Message) -> CommandResult {
    let mut message = MessageBuilder::new();
    message.push_bold("(beta) Commands\n\n").push("!ping");
    let _ = msg.reply(ctx, message.build()).await;

    Ok(())
}

#[command]
async fn listadeespera(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");

    let current_date = chrono::Utc::now()
        .with_hour(21)
        .unwrap()
        .with_minute(30)
        .unwrap();

    let mixes = db
        .mix()
        .find_many(vec![
            mix::expired::equals(false),
            mix::date::lte(current_date.fixed_offset()),
        ])
        .exec()
        .await
        .unwrap();

    if !mixes.is_empty() {
        let message = MessageBuilder::new()
            .push("Lista já foi criada 🗓️.\n")
            .push("digite !cancelarlista 💀 para remover lista atual.")
            .build();

        let _ = msg.reply(ctx, message).await;
        return Ok(());
    }

    let mix = db
        .mix()
        .create(vec![mix::date::set(current_date.fixed_offset())])
        .exec()
        .await
        .expect("Erro ao criar tabela de mix");

    let message = MessageBuilder::new()
        .push(format!(
            "Lista de espera para o dia **{}** e hora **{}** criada com êxito 🗓️.\n\n",
            mix.date.format("%d/%m"),
            mix.date.format("%H:%M")
        ))
        .push(format!("{}", env::var("DISCORD_MIX_CARGO").expect("ERR")))
        .push("digite !entrar para entrar na lista de espera")
        .build();

    let _ = msg.reply(ctx, message).await;

    Ok(())
}

#[command]
async fn sair(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");
    let mix = db
        .mix()
        .find_first(vec![mix::expired::equals(false)])
        .order_by(mix::created_at::order(Direction::Desc))
        .exec()
        .await
        .unwrap();

    if mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let player = db
        .mix_player()
        .find_first(vec![mix_player::discord_id::equals(
            msg.author.id.to_string(),
        )])
        .exec()
        .await
        .unwrap();

    let mut players = db
        .mix_player()
        .find_many(vec![mix_player::mix_id::equals(Some(
            mix.clone().unwrap().id,
        ))])
        .exec()
        .await
        .unwrap();

    let mut message = helpers::bot_make_message_mix_list(mix.clone().unwrap(), players.clone());

    if player.is_none() {
        let _ = msg
            .reply(
                ctx,
                message
                    .push("Você não está na lista 😑\n\n")
                    .push("digite **!entrar** para entrar na lista")
                    .build(),
            )
            .await;

        return Ok(());
    }

    let _ = db
        .mix_player()
        .delete(mix_player::UniqueWhereParam::IdEquals(
            player.clone().unwrap().id,
        ))
        .exec()
        .await;

    // remover cargo do author
    let _ = helpers::bot_get_member(msg.guild_id.unwrap(), ctx, msg.author.id)
        .await
        .remove_role(
            ctx,
            RoleId(
                env::var("DISCORD_LIST_CARGO_U64")
                    .expect("err")
                    .parse::<u64>()
                    .expect("err"),
            ),
        )
        .await;

    players.retain(|p| p.id != player.clone().unwrap().id);

    let mut message = helpers::bot_make_message_mix_list(mix.unwrap(), players);

    let _ = msg
        .reply(
            ctx,
            message
                .push("O bagre saiu! 🐡\n\n")
                .push("digite **!entrar** para entrar na lista")
                .build(),
        )
        .await;

    Ok(())
}

#[command]
async fn entrar(ctx: &Context, msg: &Message) -> CommandResult {
    let db = db::new_client()
        .await
        .expect("Failed to create Prisma client");
    let mix = db
        .mix()
        .find_first(vec![mix::expired::equals(false)])
        .order_by(mix::created_at::order(Direction::Desc))
        .exec()
        .await
        .unwrap();

    if mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let mut players = db
        .mix_player()
        .find_many(vec![mix_player::mix_id::equals(Some(
            mix.clone().unwrap().id,
        ))])
        .order_by(mix_player::created_at::order(Direction::Asc))
        .exec()
        .await
        .unwrap();

    let mut message = helpers::bot_make_message_mix_list(mix.clone().unwrap(), players.clone());

    for player in players.clone() {
        if player.discord_id == msg.author.id.to_string() {
            message.push("Você já está no time 💪.\n\n");
            let _ = msg.reply(ctx, message.build()).await;
            return Ok(());
        }
    }

    let player = db
        .mix_player()
        .create(
            msg.author.name.clone(),
            msg.author.id.to_string().clone(),
            vec![mix_player::mix_id::set(Some(mix.clone().unwrap().id))],
        )
        .exec()
        .await
        .expect("err ao criar players");

    players.push(player);

    let mut message = helpers::bot_make_message_mix_list(mix.clone().unwrap(), players.clone());

    // adiciona cargo para o author
    let _ = msg
        .guild_id
        .unwrap()
        .member(ctx, msg.author.id)
        .await
        .expect("asd")
        .add_role(
            ctx,
            RoleId(
                env::var("DISCORD_LIST_CARGO_U64")
                    .expect("err")
                    .parse::<u64>()
                    .expect("err"),
            ),
        )
        .await;

    // players_table.push(format!("**{}   - <@{}>**", pos + 1, player.discord_id));

    if players.len() >= 10 {
        message.push("Time ja está completo 😐.\n").push("");

        let _ = msg.reply(ctx, message.build()).await;
        return Ok(());
    }

    let _ = msg.reply(ctx, message.build()).await;

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
                    .push_bold(MAX_PLAYERS)
                    .push(" membros, ")
                    .push("na call ")
                    .mention(&channel.to_channel(ctx).await.unwrap())
                    .push(" para o sorteio. ")
                    .push("\nFaltam ")
                    .push_bold(MAX_PLAYERS - members_in_channel.len() as u8)
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
