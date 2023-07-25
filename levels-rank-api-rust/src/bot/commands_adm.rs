use std::env;

use super::bot_helper::BotHelper;
use crate::db::mix_player;
use crate::helpers::mix_helper::MixHelper;
use chrono::Timelike;
use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use serenity::model::prelude::{Message, UserId};
use serenity::model::Permissions;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[group]
#[commands(listadeespera, limparlista, cancelarlista, remover, adicionar)]
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
async fn adicionar(ctx: &Context, msg: &Message) -> CommandResult {
    let mix_helper = MixHelper::new().await;
    let bot_helper = BotHelper::new(ctx.clone());

    let current_mix = mix_helper.get_current_mix().await;

    if current_mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let mut players = mix_helper
        .get_mix_players(current_mix.clone().unwrap().id)
        .await;

    let msg_parsed: Vec<&str> = msg.content.trim().split(" ").collect();

    println!("{:?}", msg_parsed);

    if msg_parsed.len() < 2 {
        let _ = msg
            .reply(
                ctx,
                MessageBuilder::new()
                    .push("Digite uma menção. 🤨\n")
                    .push("Exemplo \"!adicionar @usuário\".")
                    .build(),
            )
            .await;
        return Ok(());
    }

    // pegar a segunda palavra da mensagem
    // que deve ser a menção
    match bot_helper.parse_mention(msg_parsed[1].to_string()) {
        Ok(m) => {
            if players
                .iter()
                .find(|p| p.discord_id == m.to_string())
                .is_none()
            {
                let member = bot_helper
                    .get_member(msg.guild_id.clone().unwrap(), ctx, UserId::from(m))
                    .await;
                let player = mix_helper
                    .create_mix_player(
                        member.user.name,
                        m.to_string(),
                        vec![mix_player::mix_id::set(Some(
                            current_mix.clone().unwrap().id,
                        ))],
                    )
                    .await;

                players.push(player);

                // adicionar cargo do usuário
                bot_helper
                    .add_member_role(
                        msg.guild_id.unwrap(),
                        UserId::from(m),
                        env::var("DISCORD_LIST_CARGO_U64").expect("err"),
                    )
                    .await;

                let mut message: MessageBuilder =
                    bot_helper.make_message_mix_list(current_mix.unwrap(), players);

                let _ = msg
                    .reply(
                        ctx,
                        message
                            .mention(&UserId::from(m))
                            .push(" foi adicionado ao mix. ")
                            .build(),
                    )
                    .await?;
                return Ok(());
            }

            let mut message = bot_helper.make_message_mix_list(current_mix.unwrap(), players);
            let _ = msg
                .reply(
                    ctx,
                    message
                        .mention(&UserId::from(m))
                        .push(" já está na lista. 😒"),
                )
                .await?;
            return Ok(());
        }
        Err(_) => {
            let _ = msg.reply(ctx, "Digite uma menção valida. 😒").await?;
        }
    }

    Ok(())
}

#[command]
async fn remover(ctx: &Context, msg: &Message) -> CommandResult {
    let mix_helper = MixHelper::new().await;
    let bot_helper = BotHelper::new(ctx.clone());

    let current_mix = mix_helper.get_current_mix().await;

    if current_mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let mut players = mix_helper
        .get_mix_players(current_mix.clone().unwrap().id)
        .await;

    let msg_parsed: Vec<&str> = msg.content.trim().split(" ").collect();

    if msg_parsed.len() < 2 {
        let _ = msg
            .reply(
                ctx,
                MessageBuilder::new()
                    .push("Mencione uma pessoa que está na lista. 🦆\n")
                    .push("Exemplo \"!remover @usuário\".")
                    .build(),
            )
            .await;
        return Ok(());
    }

    // pegar a segunda palavra da mensagem
    // que deve ser a menção
    match bot_helper.parse_mention(msg_parsed[1].to_string()) {
        Ok(m) => {
            if players
                .iter()
                .find(|p| p.discord_id == m.to_string())
                .is_some()
            {
                mix_helper
                    .delete_mix_player(m.to_string(), current_mix.clone().unwrap().id)
                    .await;

                players.retain(|p| p.discord_id != m.to_string());

                // remover cargo do usuário
                bot_helper
                    .remove_member_role(
                        msg.guild_id.unwrap(),
                        UserId::from(m),
                        env::var("DISCORD_LIST_CARGO_U64").expect("err"),
                    )
                    .await;

                let mut message: MessageBuilder =
                    bot_helper.make_message_mix_list(current_mix.unwrap(), players);

                let _ = msg
                    .reply(
                        ctx,
                        message
                            .mention(&UserId::from(m))
                            .push(" foi expulso do mix. 😂🤣")
                            .build(),
                    )
                    .await?;
                return Ok(());
            }
            let mut message = bot_helper.make_message_mix_list(current_mix.unwrap(), players);
            let _ = msg
                .reply(
                    ctx,
                    message
                        .mention(&UserId::from(m))
                        .push(" não está na lista. 😒"),
                )
                .await?;
            return Ok(());
        }
        Err(_) => {
            let _ = msg.reply(ctx, "Digite uma menção valida. 😒").await?;
        }
    }

    Ok(())
}

#[command]
async fn limparlista(ctx: &Context, msg: &Message) -> CommandResult {
    let bot_helper = BotHelper::new(ctx.clone());
    let mix_helper = MixHelper::new().await;

    let current_mix = mix_helper.get_current_mix().await;

    if current_mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let players = mix_helper
        .get_mix_players(current_mix.clone().unwrap().id)
        .await;

    // tira o cargo de todos os players da lista
    for player in players.clone() {
        let _ = bot_helper
            .remove_member_role(
                msg.guild_id.unwrap(),
                UserId::from(
                    player
                        .discord_id
                        .parse::<u64>()
                        .expect("Invalid Discord ID"),
                ),
                env::var("DISCORD_LIST_CARGO_U64").expect("err"),
            )
            .await;
    }

    // remover os players da lista
    mix_helper
        .delete_all_mix_players(current_mix.clone().unwrap().id)
        .await;

    let mut players_table = bot_helper.make_message_mix_list(current_mix.unwrap(), players.clone());

    let _ = msg.reply(ctx, players_table.build()).await;

    Ok(())
}

#[command]
async fn listadeespera(ctx: &Context, msg: &Message) -> CommandResult {
    let mix_helper = MixHelper::new().await;

    let current_date = chrono::Utc::now()
        .with_hour(21)
        .unwrap()
        .with_minute(30)
        .unwrap();

    let mixes = mix_helper
        .get_mix_many(Some(current_date.fixed_offset()))
        .await;

    if !mixes.is_empty() {
        let message = MessageBuilder::new()
            .push("Lista já foi criada 🗓️.\n")
            .push("digite !cancelarlista 💀 para remover lista atual.")
            .build();

        let _ = msg.reply(ctx, message).await;
        return Ok(());
    }

    let current_mix = mix_helper
        .create_mix(Some(current_date.fixed_offset()))
        .await;

    let message = MessageBuilder::new()
        .push(format!(
            "Lista de espera para o dia **{}** e hora **{}** criada com êxito 🗓️.\n\n",
            current_mix.date.format("%d/%m"),
            current_mix.date.format("%H:%M")
        ))
        .push(format!("{}", env::var("DISCORD_MIX_CARGO").expect("ERR")))
        .push("digite !entrar para entrar na lista de espera")
        .build();

    let _ = msg.reply(ctx, message).await;

    Ok(())
}

#[command]
async fn cancelarlista(ctx: &Context, msg: &Message) -> CommandResult {
    let mix_helper = MixHelper::new().await;
    let bot_helper = BotHelper::new(ctx.clone());

    let current_mix = mix_helper.get_current_mix().await;

    if current_mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let players = mix_helper
        .get_mix_players(current_mix.clone().unwrap().id)
        .await;

    // tira o cargo de todos os players da lista
    for player in players.clone() {
        bot_helper
            .remove_member_role(
                msg.guild_id.unwrap(),
                UserId::from(
                    player
                        .discord_id
                        .parse::<u64>()
                        .expect("Invalid Discord ID"),
                ),
                env::var("DISCORD_LIST_CARGO_U64").expect("err"),
            )
            .await;
    }

    mix_helper.cancel_current_mix(current_mix.unwrap().id).await;

    let _ = msg.reply(ctx, "Lista de espera cancelada 😨").await;

    Ok(())
}
