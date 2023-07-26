pub mod bot_helper;
mod commands;
mod commands_adm;

use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler, GatewayIntents, TypeMapKey};
use serenity::Client;
use std::env;
use tokio_cron_scheduler::JobScheduler;

struct Bot;

pub struct Cron;

impl TypeMapKey for Cron {
    type Value = JobScheduler;
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _context: Context, msg: Message) {
        println!("{}", msg.content);
        info!("{}", msg.author.name);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{}: {} is connected!",
            chrono::Utc::now().to_string(),
            ready.user.name
        );
    }
}

pub async fn serenity_instance() -> Client {
    let token = env::var("DISCORD_TOKEN").expect("token");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&commands::GENERAL_GROUP)
        .group(&commands_adm::ADMINS_GROUP);

    let client = Client::builder(token, GatewayIntents::all())
        .event_handler(Bot)
        .framework(framework)
        .await
        .expect("Error creating client");

    let cron: JobScheduler = JobScheduler::new().await.expect("asd");

    let _ = cron.start().await;

    {
        let mut data = client.data.write().await;
        data.insert::<Cron>(cron);
    }

    client
}

pub async fn serenity_start() {
    let mut client: Client = serenity_instance().await;
    let _ = client.start().await;
}
