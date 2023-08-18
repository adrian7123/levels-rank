use chrono::{Datelike, Timelike};
use color_print::cprintln;
use db::mix_schedule;
use serenity::{
    model::prelude::Message,
    prelude::{Context, TypeMapKey},
    utils::MessageBuilder,
};
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

use crate::mix_helper;

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
    pub fn date_to_schedule(date: chrono::DateTime<chrono::FixedOffset>) -> String {
        format!(
            "{} {} {} {} {} {}",
            date.second(),
            date.minute(),
            date.hour() + 3,
            date.day(),
            date.month(),
            date.weekday()
        )
    }
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

        let schedule: String = Self::date_to_schedule(current_date);

        let uuid = self
            .cron
            .add(
                Job::new_cron_job_async(schedule.as_str(), move |uuid, _lock| {
                    let http = ctx.http.clone();
                    let m = message.clone();

                    Box::pin(async move {
                        cprintln!(
                            "<yellow><bold>Cron</bold>send_message_discord at: {}</>",
                            chrono::Utc::now()
                        );

                        let _ = msg.channel_id.say(http.as_ref(), m).await;

                        mix_helper::MixHelper::new()
                            .await
                            .update_mix_schedule(
                                uuid.to_string(),
                                vec![mix_schedule::executed::set(true)],
                            )
                            .await;
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
