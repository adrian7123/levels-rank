use chrono::{DateTime, FixedOffset, Timelike};
use prisma_client_rust::Direction;
use serenity::utils::MessageBuilder;

use db::{self, mix, mix_player, mix_schedule};

#[derive(Debug)]
pub struct MixHelper {
    db: db::PrismaClient,
}

impl MixHelper {
    pub async fn new() -> Self {
        Self {
            db: db::new_client()
                .await
                .expect("Failed to create Prisma client"),
        }
    }
    pub fn make_message_mix_list(
        &self,
        mix: mix::Data,
        players: Vec<mix_player::Data>,
    ) -> MessageBuilder {
        let mut message: MessageBuilder = MessageBuilder::new();

        message
            .push("Mix Que Ota Community ")
            .push(mix.date.format("**%d/%m** "))
            .push(mix.date.format("**%H:%M** "))
            .push("\n\n");
        let mut pos: u8 = 0;
        for player in players {
            pos += 1;
            message.push_bold(format!("{}  -  <@{}>", pos, player.discord_id));
            message.push("\n");
        }

        message.push("\n");

        message
    }
    pub fn get_current_date(&self, hour: Option<u32>, min: Option<u32>) -> DateTime<FixedOffset> {
        let mut h: u32 = 0;
        let mut m: u32 = 0;

        if hour.is_some() {
            h = hour.unwrap();
        }
        if min.is_some() {
            m = min.unwrap();
        }

        chrono::Utc::now()
            .with_hour(h)
            .unwrap()
            .with_minute(m)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .fixed_offset()
    }
    pub async fn mix_is_created(&self) -> (bool, MessageBuilder) {
        let mixes = self.get_mix_many().await;
        let mut message = MessageBuilder::new();
        let mut created = false;
        if mixes.is_empty() {
            message
                .push("Lista de espera ainda não foi criada 😐")
                .push("Digite !criarlista **22:00** para criar uma nova lista.📅");
        } else {
            created = true;
            message
                .push("Lista já foi criada 🗓️.\n")
                .push("Digite !cancelarlista 💀 para remover lista atual.")
                .build();
        }
        (created, message)
    }
    pub async fn create_mix(&self, current_date: Option<DateTime<FixedOffset>>) -> mix::Data {
        self.db
            .mix()
            .create(vec![mix::date::set(current_date.unwrap())])
            .exec()
            .await
            .unwrap()
    }
    /// Save cronjob of mix
    pub async fn create_mix_schedule(&self, mix_id: String, uuid: String, schedule: String) {
        self.db
            .mix_schedule()
            .create(
                uuid,
                mix::UniqueWhereParam::IdEquals(mix_id),
                vec![mix_schedule::schedule::set(schedule)],
            )
            .exec()
            .await
            .unwrap();
    }
    /// Update cronjob of mix
    pub async fn update_mix_schedule(&self, uuid: String, params: Vec<mix_schedule::SetParam>) {
        self.db
            .mix_schedule()
            .update(mix_schedule::id::equals(uuid), params)
            .exec()
            .await
            .unwrap();
    }
    pub async fn get_mix_many(&self) -> Vec<mix::Data> {
        let where_params = vec![mix::expired::equals(false)];

        self.db.mix().find_many(where_params).exec().await.unwrap()
    }
    pub async fn get_current_mix(&self) -> Option<mix::Data> {
        self.db
            .mix()
            .find_first(vec![mix::expired::equals(false)])
            .order_by(mix::created_at::order(Direction::Desc))
            .exec()
            .await
            .unwrap()
    }
    pub async fn get_mix_players(&self, mix_id: String) -> Vec<mix_player::Data> {
        self.db
            .mix_player()
            .find_many(vec![mix_player::mix_id::equals(Some(mix_id))])
            .order_by(mix_player::created_at::order(Direction::Asc))
            .exec()
            .await
            .unwrap()
    }
    pub async fn create_mix_player(
        &self,
        name: String,
        discord_id: String,
        create_params: Vec<mix_player::SetParam>,
    ) -> mix_player::Data {
        self.db
            .mix_player()
            .create(name, discord_id, create_params)
            .exec()
            .await
            .expect("err ao criar player")
    }
    pub async fn get_mix_player(
        &self,
        _where: Vec<mix_player::WhereParam>,
    ) -> Option<mix_player::Data> {
        self.db
            .mix_player()
            .find_first(_where)
            .exec()
            .await
            .unwrap()
    }
    pub async fn cancel_current_mix(&self, mix_id: String) {
        let _ = self
            .db
            .mix()
            .update(mix::id::equals(mix_id), vec![mix::expired::set(true)])
            .exec()
            .await;
    }
    pub async fn delete_all_mix_players(&self, mix_id: String) {
        self.db
            .mix_player()
            .delete_many(vec![mix_player::mix_id::equals(Some(mix_id))])
            .exec()
            .await
            .unwrap();
    }

    pub async fn delete_mix_player(&self, discord_id: String, mix_id: String) {
        self.db
            .mix_player()
            .delete_many(vec![
                mix_player::discord_id::equals(discord_id),
                mix_player::mix_id::equals(Some(mix_id)),
            ])
            .exec()
            .await
            .unwrap();
    }
}
