use std::{future::Future, pin::Pin};

use chrono::{Datelike, Duration, Timelike};
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

pub struct CronHelper {
    pub cron: JobScheduler,
}

impl CronHelper {
    pub fn date_to_schedule(date: chrono::DateTime<chrono::FixedOffset>) -> String {
        let mut new_date = date + Duration::hours(3);

        new_date = new_date.with_day(date.day()).unwrap();
        new_date = new_date.with_month(date.month()).unwrap();
        new_date = new_date.with_year(date.year()).unwrap();

        format!(
            "{} {} {} {} {} {}",
            new_date.second(),
            new_date.minute(),
            new_date.hour(),
            new_date.day(),
            new_date.month(),
            new_date.weekday()
        )
    }
    pub async fn new_by_discord(ctx: &Context) -> Self {
        let data = ctx.data.read().await;
        let cron = data.get::<Cron>().unwrap();

        Self { cron: cron.clone() }
    }
    pub async fn add<T>(
        &self,
        current_date: chrono::DateTime<chrono::FixedOffset>,
        run: T,
    ) -> Result<(Uuid, String), String>
    where
        T: 'static
            + FnMut(Uuid, JobScheduler) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync,
    {
        let schedule: String = Self::date_to_schedule(current_date);

        let job = Job::new_cron_job_async(schedule.as_str(), run);

        match job {
            Ok(job) => {
                let uuid = self.cron.add(job).await;

                match uuid {
                    Ok(uuid) => return Ok((uuid, schedule)),
                    Err(e) => return Err(format!("Cron not created: {:?}", e.to_string())),
                }
            }
            Err(e) => return Err(format!("Job not created: {:?}", e.to_string())),
        }
    }
    pub async fn send_message_discord(
        &self,
        current_date: chrono::DateTime<chrono::FixedOffset>,
        ctx: Context,
        msg: Message,
        message_builder: &mut MessageBuilder,
    ) -> Result<(Uuid, String), String> {
        let message = message_builder.build();

        let schedule: String = Self::date_to_schedule(current_date);

        let job = Job::new_cron_job_async(schedule.as_str(), move |uuid, _lock| {
            let http = ctx.http.clone();
            let m = message.clone();

            Box::pin(async move {
                cprintln!(
                    "<yellow><bold>Cron</bold> send_message_discord at: {}</>",
                    chrono::Utc::now()
                );

                let _ = msg.channel_id.say(http.as_ref(), m).await;

                mix_helper::MixHelper::new()
                    .await
                    .update_mix_schedule(uuid.to_string(), vec![mix_schedule::executed::set(true)])
                    .await;
            })
        });

        match job {
            Ok(job) => {
                let uuid = self.cron.add(job).await;

                match uuid {
                    Ok(uuid) => return Ok((uuid, schedule)),
                    Err(e) => return Err(format!("Cron not created: {:?}", e.to_string())),
                }
            }
            Err(e) => return Err(format!("Job not created: {:?}", e.to_string())),
        }
    }
    pub async fn cancel_all_cron_from_mix(&self, mix_id: String) {
        let mix_helper = mix_helper::MixHelper::new().await;

        let mix_schedules = mix_helper.get_mix_schedule_many_by_mix_id(mix_id).await;

        println!("mix_schedules: {:?}", mix_schedules);

        for mix_schedule in mix_schedules {
            // remove do cronjob
            let _ = self
                .cron
                .remove(&Uuid::parse_str(mix_schedule.id.as_str()).unwrap())
                .await;

            // salva executado na tabela
            mix_helper
                .update_mix_schedule(mix_schedule.id, vec![mix_schedule::executed::set(true)])
                .await;
        }
    }
    pub async fn shutdown(&self) {
        let _ = self.cron.clone().shutdown().await;
    }
}
