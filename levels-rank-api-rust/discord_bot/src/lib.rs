pub mod bot_helper;
mod commands;
mod commands_adm;
mod tables;

use color_print::cprintln;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::prelude::Ready;
use serenity::prelude::{Context, EventHandler, GatewayIntents};
use serenity::{async_trait, Client};
use shared::cron_helper::Cron;
use tokio_cron_scheduler::JobScheduler;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _context: Context, msg: Message) {
        println!("{} \x1b[1;37m@{}\x1b[0m", msg.content, msg.author.name);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        cprintln!("<green>{} is connected!</green>", ready.user.name);
    }
}

pub async fn serenity_instance(token: String) -> Client {
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

pub async fn serenity_start(token: String) {
    let mut client: Client = serenity_instance(token).await;
    let _ = client.start().await;
}
