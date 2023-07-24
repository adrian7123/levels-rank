pub mod bot_helper;
mod commands;
mod commands_adm;

use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler, GatewayIntents};
use serenity::Client;
use std::env;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _context: Context, msg: Message) {
        println!("{}", msg.content);
        info!("{}", msg.author.name);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub async fn serenity_instance() -> Client {
    let token = env::var("DISCORD_TOKEN").expect("token");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&commands::GENERAL_GROUP)
        .group(&commands_adm::ADMINS_GROUP);

    Client::builder(token, GatewayIntents::all())
        .event_handler(Bot)
        .framework(framework)
        .await
        .expect("Error creating client")
}

pub async fn serenity_start() {
    let mut client: Client = serenity_instance().await;
    let _ = client.start().await;
}
