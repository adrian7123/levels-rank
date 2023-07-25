use chrono::{DateTime, FixedOffset};
use prisma_client_rust::Direction;

use crate::db::{
    self,
    mix::{self},
    mix_player::{self},
};

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
    pub async fn create_mix(&self, current_date: Option<DateTime<FixedOffset>>) -> mix::Data {
        self.db
            .mix()
            .create(vec![mix::date::set(current_date.unwrap())])
            .exec()
            .await
            .unwrap()
    }
    pub async fn get_mix_many(
        &self,
        current_date: Option<DateTime<FixedOffset>>,
    ) -> Vec<mix::Data> {
        let mut where_params = vec![mix::expired::equals(false)];

        if current_date.is_some() {
            where_params.push(mix::date::lte(current_date.unwrap()));
        }

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
