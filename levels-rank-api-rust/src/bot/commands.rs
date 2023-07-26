use std::env;

use super::bot_helper::BotHelper;
use crate::db::mix_player;
use crate::helpers::mix_helper::MixHelper;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::{ChannelId, Guild, Member, Message};
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use tabled::settings::Style;
use tabled::{Table, Tabled};

const MAX_PLAYERS: u8 = 10;

#[group]
#[commands(ping, times, entrar, comandos, sair, lista)]
pub struct General;

#[command]
async fn lista(ctx: &Context, msg: &Message) -> CommandResult {
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

    let mut players_table = bot_helper.make_message_mix_list(current_mix.unwrap(), players.clone());

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
async fn sair(ctx: &Context, msg: &Message) -> CommandResult {
    let bot_helper = BotHelper::new(ctx.clone());
    let mix_helper = MixHelper::new().await;

    let current_mix = mix_helper.get_current_mix().await;

    if current_mix.is_none() {
        let _ = msg
            .reply(ctx, "Lista de espera ainda não foi criada 😐")
            .await;

        return Ok(());
    }

    let player = mix_helper
        .get_mix_player(vec![
            mix_player::discord_id::equals(msg.author.id.to_string()),
            mix_player::mix_id::equals(Some(current_mix.clone().unwrap().id)),
        ])
        .await;

    let mut players = mix_helper
        .get_mix_players(current_mix.clone().unwrap().id)
        .await;

    let mut message =
        bot_helper.make_message_mix_list(current_mix.clone().unwrap(), players.clone());

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

    let _ = mix_helper
        .delete_mix_player(
            player.clone().unwrap().discord_id,
            current_mix.clone().unwrap().id,
        )
        .await;

    // remover cargo do author
    bot_helper
        .remove_member_role(
            msg.guild_id.unwrap(),
            msg.author.id,
            env::var("DISCORD_LIST_CARGO_U64").expect("err"),
        )
        .await;

    players.retain(|p| p.id != player.clone().unwrap().id);

    let mut message = bot_helper.make_message_mix_list(current_mix.unwrap(), players);

    let _ = msg
        .reply(
            ctx,
            message
                .push("O bagre saiu! 🐡\n")
                .mention(&msg.author.id)
                .push("\n\ndigite **!entrar** para entrar na lista")
                .build(),
        )
        .await;

    Ok(())
}

#[command]
async fn entrar(ctx: &Context, msg: &Message) -> CommandResult {
    let bot_helper = BotHelper::new(ctx.clone());
    let mix_helper = MixHelper::new().await;

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

    let mut message =
        bot_helper.make_message_mix_list(current_mix.clone().unwrap(), players.clone());

    for player in players.clone() {
        if player.discord_id == msg.author.id.to_string() {
            message.push("Você já está no time 💪.\n\n");
            let _ = msg.reply(ctx, message.build()).await;
            return Ok(());
        }
    }

    let player = mix_helper
        .create_mix_player(
            msg.author.name.clone(),
            msg.author.id.to_string().clone(),
            vec![mix_player::mix_id::set(Some(
                current_mix.clone().unwrap().id,
            ))],
        )
        .await;

    players.push(player);

    let mut message =
        bot_helper.make_message_mix_list(current_mix.clone().unwrap(), players.clone());

    // adiciona cargo para o author
    bot_helper
        .add_member_role(
            msg.guild_id.unwrap(),
            msg.author.id,
            env::var("DISCORD_LIST_CARGO_U64").expect("err"),
        )
        .await;

    if players.len() >= 10 {
        message.push("Time ja está completo 😐.\n").push("");

        let _ = msg.reply(ctx, message.build()).await;
        return Ok(());
    }

    let _ = msg.reply(ctx, message.build()).await;

    Ok(())
}

#[derive(Tabled)]
struct TimesTable {
    contra_terrorista: String,
    terrorista: String,
}

#[command]
async fn times(ctx: &Context, msg: &Message) -> CommandResult {
    let bot_helper = BotHelper::new(ctx.clone());

    let channel_ruffle_id = env::var("DISCORD_RUFFLE_CHANNEL")
        .expect("err DISCORD_LOG_CHANNEL")
        .parse::<u64>()
        .expect("err parse discord channel id");

    let guild: Guild = msg.guild_id.expect("err").to_guild_cached(ctx).unwrap();

    let channel: ChannelId = ChannelId(channel_ruffle_id);

    let members_in_channel: Vec<Member> = bot_helper.members_in_channel(guild, channel);

    for member in &members_in_channel {
        println!(
            "Member ID: {}, Member Name: {} {}",
            member.user.id,
            member.user.name,
            &members_in_channel.len()
        );
    }

    if members_in_channel.len() < 10 {
        let _ = msg
            .reply(
                &ctx.http,
                MessageBuilder::new()
                    .push("Necessário ")
                    .push_bold(MAX_PLAYERS)
                    .push(" membros, ")
                    .push("na call ")
                    .mention(&channel.to_channel(ctx).await.unwrap())
                    .push(" para o sorteio.\n")
                    .push("Faltam ")
                    .push_bold(MAX_PLAYERS - members_in_channel.len() as u8)
                    .build(),
            )
            .await;

        return Ok(());
    }

    let member_names: Vec<String> = members_in_channel
        .iter()
        .map(|member| member.display_name().to_string())
        .collect();

    let half = member_names.len() / 2;
    let mut rng = StdRng::from_entropy();

    let ct: Vec<String> = member_names
        .choose_multiple(&mut rng, half)
        .cloned()
        .collect();

    let tr: Vec<String> = member_names
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
