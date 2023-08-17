use chrono::{Datelike, Timelike};
use color_print::cprintln;
use serenity::{
    model::prelude::Message,
    prelude::{Context, TypeMapKey},
    utils::MessageBuilder,
};
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

pub struct Cron;

impl TypeMapKey for Cron {
    type Value = JobScheduler;
}

#[allow(dead_code)]
pub struct CronHelper {
    pub cron: JobScheduler,
}

#[allow(dead_code)]
impl CronHelper {
    pub async fn new_by_discord(ctx: &Context) -> Self {
        let data = ctx.data.read().await;
        let cron = data.get::<Cron>().unwrap();

        Self { cron: cron.clone() }
    }
    pub async fn add<T>(&self, schedule: &str, run: T)
    where
        T: 'static + FnMut(Uuid, JobScheduler) + Send + Sync,
    {
        let _ = self.cron.add(Job::new(schedule, run).expect("msg")).await;
    }
    pub async fn send_message_discord(
        &self,
        current_date: chrono::DateTime<chrono::FixedOffset>,
        ctx: Context,
        msg: Message,
        message_builder: &mut MessageBuilder,
    ) -> (Uuid, String) {
        let message = message_builder.build();

        let schedule: String = format!(
            "{} {} {} {} {} {}",
            current_date.second(),
            current_date.minute() + 1,
            current_date.hour(),
            current_date.day(),
            current_date.month(),
            current_date.weekday()
        );

        let uuid = self
            .cron
            .add(
                Job::new_cron_job_async(schedule.as_str(), move |_uuid, _lock| {
                    let http = ctx.http.clone();
                    let m = message.clone();

                    Box::pin(async move {
                        cprintln!(
                            "<yellow><bold>Cron</bold>send_message_discord at: {}</>",
                            chrono::Utc::now()
                        );

                        let _ = msg.channel_id.say(http.as_ref(), m).await;
                    })
                })
                .expect("msg"),
            )
            .await
            .expect("err");

        (uuid, schedule)
    }
    pub async fn shutdown(&self) {
        let _ = self.cron.clone().shutdown().await;
    }
}
