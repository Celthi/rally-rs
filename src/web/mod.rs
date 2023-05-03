use crate::{config_env, token::db};
use anyhow::Result;
use poem::{
    handler, listener::TcpListener, middleware::Tracing, post, web::Json, EndpointExt, Route,
    Server,
};
use serde::Deserialize;
use tracing::error;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

#[tokio::main]
pub async fn event_loop() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    let router = Route::new();
    let app = router
        .at("/rally_token", post(save_rally_token))
        .with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:30814"))
        .run(app)
        .await
}
#[derive(Debug, Deserialize)]
pub struct RallyTokenUpdate {
    user_name: String,
    token: String,
}
#[handler(method = "post")]
async fn save_rally_token(req: Json<RallyTokenUpdate>) {
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
