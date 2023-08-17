use std::env;

use super::bot_helper::BotHelper;
use super::tables::TimesTable;
use chrono::Timelike;
use db::mix_player;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serenity::framework::standard::macros::{check, command, group};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use serenity::model::prelude::*;
use serenity::model::Permissions;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use shared::constants::MAX_PLAYERS;
use shared::cron_helper::CronHelper;
use shared::mix_helper::MixHelper;
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

#[command]
async fn criarlista(ctx: &Context, msg: &Message) -> CommandResult {
    let mut hour: u32 = 21;
    let mut min: u32 = 30;

    let mix_helper: MixHelper = MixHelper::new().await;

    let msg_parsed: Vec<&str> = msg.content.trim().split(" ").collect();

    if let [_, hour_str] = msg_parsed.as_slice() {
        let hour_arr: Vec<&str> = hour_str.split(":").collect();

        if hour_arr.len() >= 2 {
            hour = hour_arr[0].parse().expect("msg");
            min = hour_arr[1].parse().expect("msg");
        } else {
            let _ = msg
                .reply(
                    ctx,
                    "Formato de Hora incorreto\nExemplo !criarlista **22:00**",
                )
                .await;
        }
    }

    let (created, message) = mix_helper.mix_is_created().await;

    if created {
        let _ = msg.reply(ctx, message).await;

        return Ok(());
    }

    let current_date = mix_helper.get_current_date(Some(hour), Some(min));

    let current_mix = mix_helper.create_mix(Some(current_date)).await;

    let cron_helper = CronHelper::new_by_discord(ctx).await;

    let mix_cargo = env::var("DISCORD_MIX_CARGO").expect("ERR");

    let messages: Vec<String> = vec![
        format!("{} Faltam 30 minutos, fiquem atentos!🙈", mix_cargo),
        format!("{} Faltam 15 minutos, fiquem atentos!🙊", mix_cargo),
        format!("{} É agora!🥶", mix_cargo),
    ];
    let mut c = 0;

    for message in messages {
        current_date.with_minute(current_date.minute() + c);
        c += 1;

        match cron_helper
            .send_message_discord(
                current_date,
                ctx.clone(),
                msg.clone(),
                MessageBuilder::new().push(message),
            )
            .await
        {
            (uuid, schedule) => {
                mix_helper
                    .create_mix_schedule(current_mix.clone().id, uuid.to_string(), schedule)
                    .await;
            }
        }
    }

    let message = MessageBuilder::new()
        .push(format!(
            "Lista de espera para o dia **{}** e hora **{}** criada com êxito 🗓️.\n\n",
            current_mix.date.format("%d/%m"),
            current_mix.date.format("%H:%M")
        ))
        .push(format!("{}", mix_cargo))
        .push("digite !entrar para entrar na lista de espera")
        .build();

    let _ = msg.reply(ctx, message).await;

    Ok(())
}

#[command]
async fn sortearlista(ctx: &Context, msg: &Message) -> CommandResult {
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

    if players.len() % 2 != 0 {
        let _ = msg
            .reply(
                ctx,
                MessageBuilder::new()
                    .push("Necessário um numero par para sortear os times 😒")
                    .build(),
            )
            .await;
        return Ok(());
    }

    let player_ids: Vec<String> = players
        .iter()
        .map(|player| player.discord_id.clone())
        .collect();

    let player_names: Vec<String> = bot_helper
        .get_members_by_ids(msg.guild_id.unwrap(), player_ids)
        .await
        .iter()
        .map(|member| member.nick.clone().unwrap())
        .collect();

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

    let (created, message) = mix_helper.mix_is_created().await;

    if !created {
        let _ = msg.reply(ctx, message).await?;
        return Ok(());
    }

    let current_mix = mix_helper.get_current_mix().await;

    let mut players = mix_helper
        .get_mix_players(current_mix.clone().unwrap().id)
        .await;

    let mut message: MessageBuilder =
        mix_helper.make_message_mix_list(current_mix.clone().unwrap(), players.clone());

    if (players.len() as u8) >= MAX_PLAYERS {
        message.push("Time ja está completo 😐.\n").push("");

        let _ = msg.reply(ctx, message.build()).await;
        return Ok(());
    }

    let msg_parsed: Vec<&str> = msg.content.trim().split(" ").collect();

    println!("{:?}", msg_parsed);

    if msg_parsed.len() < 2 {
        let _ = msg
            .reply(
                ctx,
                MessageBuilder::new()
                    .push("Digite pelo menos uma menção. 🤨\n")
                    .push("Exemplo \"!adicionar @usuário1 @usuário2\".")
                    .build(),
            )
            .await;
        return Ok(());
    }

    for mention in msg_parsed {
        if mention.starts_with("!") {
            continue;
        }
        match bot_helper.parse_mention(mention.to_string()) {
            Ok(m) => {
                if players
                    .clone()
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
                    continue;
                }
            }
            Err(_) => {
                let _ = msg
                    .reply(
                        ctx,
                        MessageBuilder::new()
                            .push(mention.to_string())
                            .push(" não é uma menção valida. 😐"),
                    )
                    .await?;
            }
        }
    }

    let mut message: MessageBuilder =
        mix_helper.make_message_mix_list(current_mix.clone().unwrap(), players.clone());

    let _ = msg.reply(ctx, message.build()).await?;
    Ok(())
}

#[command]
async fn remover(ctx: &Context, msg: &Message) -> CommandResult {
    let mix_helper = MixHelper::new().await;
    let bot_helper = BotHelper::new(ctx.clone());

    let (created, message) = mix_helper.mix_is_created().await;

    if !created {
        let _ = msg.reply(ctx, message).await?;
        return Ok(());
    }

    let current_mix = mix_helper.get_current_mix().await;

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
                    .push("Exemplo \"!remover @usuário1 @usuário2\".")
                    .build(),
            )
            .await;
        return Ok(());
    }
    for mention in msg_parsed {
        if mention.starts_with("!") {
            continue;
        }
        match bot_helper.parse_mention(mention.to_string()) {
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

                    continue;
                }
            }
            Err(_) => {
                let _ = msg
                    .reply(
                        ctx,
                        MessageBuilder::new()
                            .push(mention.to_string())
                            .push(" não é uma menção valida. 😒")
                            .build(),
                    )
                    .await?;
            }
        }
    }
    let message: MessageBuilder = mix_helper.make_message_mix_list(current_mix.unwrap(), players);

    let _ = msg.reply(ctx, message).await?;
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

    let mut message = mix_helper.make_message_mix_list(current_mix.unwrap(), players);

    message.push("Lista limpa com sucesso! broxa 🐐");

    let _ = msg.reply(ctx, message.build()).await;

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
