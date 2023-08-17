use serenity::prelude::{Context, TypeMapKey};
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
    pub async fn shutdown(&self) {
        let _ = self.cron.clone().shutdown().await;
    }
}
