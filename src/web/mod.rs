use crate::{config_env, token::db};
use anyhow::Result;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use axum::{routing::post, Json, Router};

use serde::Deserialize;
use tracing::error;

#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    // build our application with a route
    let app = Router::new().route("/rally_token", post(save_rally_token));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:31430").await?;
    axum::serve(listener, app).await
}

#[derive(Debug, Deserialize)]
pub struct RallyTokenUpdate {
    user_name: String,
    token: String,
}

async fn save_rally_token(Json(req): Json<RallyTokenUpdate>) {
    match insert_to_db(&req).await {
        Ok(()) => {}
        Err(e) => {
            error!("{:?}", e)
        }
    }
}

async fn insert_to_db(u: &RallyTokenUpdate) -> Result<()> {
    let mc = new_magic_crypt!(config_env::get_encrypt_key(), 256);
    let mut db = db::DB::new(
        config_env::get_db_host(),
        config_env::get_db_user(),
        config_env::get_db_password(),
        config_env::get_db_port(),
    )
    .await
    .unwrap();
    let base64 = mc.encrypt_str_to_base64(&u.token);

    db.insert_rally_token(&u.user_name, &base64).await
}
