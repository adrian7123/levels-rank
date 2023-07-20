use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::prelude::{ChannelId, Ready};
use serenity::utils::MessageBuilder;
use std::env;

use serenity::prelude::*;

pub async fn bot_send_message(
    str_channel_id: String,
    http: &Http,
    message_builder: &mut MessageBuilder,
) -> () {
    match str_channel_id.parse::<u64>() {
        Ok(channel_id) => {
            let response = message_builder.build();

            if let Err(why) = ChannelId(channel_id).say(http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
        Err(why) => {
            return eprintln!("Erro ao enviar mensagem {}", why);
        }
    }
}

#[group]
struct General;
struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, context: Context, msg: Message) {
        println!("{}", msg.content);
        info!("{}", msg.author.name);
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {:?}", why);

                    return;
                }
            };

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

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn serenity_start() {
    let mut client: Client = serenity_instance().await;
    let _ = client.start().await;
}

pub async fn serenity_instance() -> Client {
    let token = env::var("DISCORD_TOKEN").expect("token");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    Client::builder(
        token,
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Bot)
    .framework(framework)
    .await
    .expect("Error creating client")
}
