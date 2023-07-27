use std::env;

use super::bot_helper::BotHelper;
use super::tables::TimesTable;
use crate::db::mix_player;
use crate::helpers::constants::MAX_PLAYERS;
use crate::helpers::mix_helper::MixHelper;
use chrono::Timelike;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use serenity::model::prelude::{Message, UserId};
use serenity::model::Permissions;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tabled::settings::Style;
use tabled::Table;
use tokio_cron_scheduler::{Job, JobScheduler};

#[group]
#[commands(
    criarlista,
    limparlista,
    cancelarlista,
    remover,
    adicionar,
    sortearlista
)]
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
        println!("@{} não tem cargo para esse comando 🤨", msg.author.name);
        Err(Reason::Log(
            "You don't have the required role to use this command.".to_string(),
        ))
    }
}

#[command]
async fn sortearlista(ctx: &Context, msg: &Message) -> CommandResult {
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

    if (players.len() as u8) < MAX_PLAYERS {
        let _ = msg
            .reply(
                &ctx.http,
                MessageBuilder::new()
                    .push("Necessário ")
                    .push_bold(MAX_PLAYERS)
                    .push(" na lista, ")
                    .push(" para o sorteio.\n")
                    .push("Faltam ")
                    .push_bold(MAX_PLAYERS - players.len() as u8)
                    .build(),
            )
            .await;

        return Ok(());
    }

    let player_names: Vec<String> = players.iter().map(|player| player.name.clone()).collect();

    let half = player_names.len() / 2;
    let mut rng = StdRng::from_entropy();

    let ct: Vec<String> = player_names
        .choose_multiple(&mut rng, half)
        .cloned()
        .collect();

    let tr: Vec<String> = player_names
        .iter()
        .filter(|member_name| !ct.contains(member_name))
        .cloned()
        .collect();

    let mut times: Vec<TimesTable> = vec![];

    for i in 0..half {
        times.push(TimesTable {
            contra_terrorista: ct[i].clone(),
            terrorista: tr[i].clone(),
        })
    }

    let table = Table::new(times).with(Style::modern()).to_string();

    let _ = msg
        .reply(
            ctx,
            MessageBuilder::new()
                .push(format!("```{}```", table))
                .build(),
        )
        .await;
    Ok(())
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

    let mut cron = JobScheduler::new().await.expect("JobScheduler::new()");

    let _ = cron
        .add(
            Job::new("1/10 * * * * *", |_, __| {
                println!("I get executed every 10 seconds!");
            })
            .expect("msg"),
        )
        .await;

    let _ = cron.remove_shutdown_handler();

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

    players.retain(|p| p.mix_id.clone().unwrap() != current_mix.clone().unwrap().id.to_string());

    let mut message = bot_helper.make_message_mix_list(current_mix.unwrap(), players);

    message.push("Lista limpa com sucesso! broxa 🐐");

    let _ = msg.reply(ctx, message.build()).await;

    Ok(())
}

#[command]
async fn criarlista(ctx: &Context, msg: &Message) -> CommandResult {
    let mut hour: u32 = 21;
    let mut min: u32 = 30;

    let mix_helper = MixHelper::new().await;

    let msg_parsed: Vec<&str> = msg.content.trim().split(" ").collect();

    if msg_parsed.len() >= 2 {
        let hour_arr: Vec<&str> = msg_parsed[1].split(":").collect();

        if hour_arr.len() == 2 {
            hour = hour_arr[0].parse().expect("msg");
            min = hour_arr[1].parse().expect("msg");
        }
    }

    let current_date = chrono::Utc::now()
        .with_hour(hour)
        .unwrap()
        .with_minute(min)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
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
