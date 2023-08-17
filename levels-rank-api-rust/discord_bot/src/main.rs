// ===========================
// ARQUIVO MAIN PARA TESTAR A LIB
//

use discord_bot::serenity_start;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    serenity_start(env::var("DISCORD_TOKEN").expect("token")).await;
}
