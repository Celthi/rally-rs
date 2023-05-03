use crate::config_env;
use crate::token::db::DB;
use anyhow::Result;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use tracing::error;

pub struct UserToken {
    pub name: String,
    pub token: String,
}

impl UserToken {
    pub fn new(name: String, token: String) -> Self {
        UserToken { name, token }
    }
}

pub async fn get_rally_token(user_name: &str) -> Result<UserToken> {
    let mut db = DB::new(
        config_env::get_db_host(),
        config_env::get_db_user(),
        config_env::get_db_password(),
        config_env::get_db_port(),
    )
    .await?;
    let s = db.get_rally_token(user_name).await?;
    if s.is_some() {
        let mc = new_magic_crypt!(config_env::get_encrypt_key(), 256);
        if let Ok(s) = mc.decrypt_base64_to_string(s.unwrap()) {
            Ok(UserToken::new(user_name.to_string(), s))
        } else {
            Err(anyhow::anyhow!("token not correctly encrypted!"))
        }
    } else {
        let error = format!("Token not found for {}", user_name);
        error!(error);
        Err(anyhow::anyhow!(error))
    }
}
