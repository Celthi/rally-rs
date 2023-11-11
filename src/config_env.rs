use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use std::env;
use std::process;

#[derive(Debug)]
pub struct ConfigEnv {
    pub db_host: String,
    pub db_port: Option<String>,
    pub db_user: String,
    pub db_password: String,
    pub github_token: String,
    pub kafka_broker_list: String,
    pub time_spent_topic: String,
    pub consumer_group_id: String,
    pub encrypt_key: String,
    pub github_url: String,
    pub doc_link: String,
    pub rally_url: String,
    pub workspace_id: String,
    pub root_project_id: String,
}

impl ConfigEnv {
    pub fn new() -> Result<ConfigEnv> {
        let Ok(db_host) = env::var("DB_HOST") else {
            return Err(anyhow!(
                "DB Host is required, please provide it by env variable DB_HOST"
            ));
        };
        let db_port = if env::var("DB_PORT").is_err() {
            Some("5432".to_string())
        } else {
            None
        };
        let Ok(db_user) = env::var("DB_USER") else {
            return Err(anyhow!(
                "DB port is required, please provide it by env variable DB_USER"
            ));
        };
        let Ok(db_password) = env::var("DB_PASSWORD") else {
            return Err(anyhow!(
                "DB port is required, please provide it by env variable DB_PASSWORD"
            ));
        };
        let Ok(github_token) = env::var("GITHUB_TOKEN") else {
            return Err(anyhow!(
                "GITHUB_TOKEN is required, please provide it by env variable GITHUB_TOKEN"
            ));
        };
        let Ok(kafka_broker_list) = env::var("KAFKA_BROKER_LIST") else {
            return Err(anyhow!(
                "KAFKA_BROKER_LIST is required, please provide it by env variable KAFKA_BROKER_LIST like localhost:9092"
            ));
        };
        let time_spent_topic = env::var("KAFKA_TP_TOPIC").unwrap_or("time_spent".to_string());
        let consumer_group_id = env::var("KAFKA_TP_TOPIC").unwrap_or("rally_consume".to_string());
        let Ok(encrypt_key) = env::var("ENCRYPT_KEY") else {
            return Err(anyhow!(
                "ENCRYPT_KEY is required, please provide it by env variable ENCRYPT_KEY"
            ));
        };
        let github_url =
            env::var("GITHUB_URL").unwrap_or_else(|_| "https://api.github.com/".to_string());
        let doc_link =
            env::var("TNT_DOC_LINK").unwrap_or_else(|_| "https://api.github.com/".to_string());
        let Ok(rally_url) = env::var("RALLY_URL") else {
            return Err(anyhow!(
                "RALLY_URL is required, please provide it by env variable RALLY_URL"
            ));
        };
        let workspace_id =
            env::var("RALLY_WORKSPACE_ID").unwrap_or_else(|_| "27397600726".to_string());
        let root_project_id =
            env::var("RALLY_ROOT_PROJECT_ID").unwrap_or_else(|_| "40120756498".to_string());

        Ok(ConfigEnv {
            db_host,
            db_port,
            db_user,
            db_password,
            github_token,
            kafka_broker_list,
            time_spent_topic,
            consumer_group_id,
            encrypt_key,
            github_url,
            doc_link,
            rally_url,
            workspace_id,
            root_project_id,
        })
    }
}

pub static CONFIG: OnceCell<ConfigEnv> = OnceCell::new();

pub fn get_db_host() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").db_host
}

pub fn get_db_port() -> Option<&'static str> {
    CONFIG
        .get()
        .expect("fail to get env variable")
        .db_port
        .as_deref()
}

pub fn get_db_user() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").db_user
}

pub fn get_db_password() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").db_password
}

pub fn get_github_token() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").github_token
}

pub fn get_kafka_broker_list() -> &'static str {
    &CONFIG
        .get()
        .expect("fail to get env variable")
        .kafka_broker_list
}

pub fn get_kafka_time_spent_topic() -> &'static str {
    &CONFIG
        .get()
        .expect("fail to get env variable")
        .time_spent_topic
}

pub fn get_consumer_group_id() -> &'static str {
    &CONFIG
        .get()
        .expect("fail to get env variable")
        .consumer_group_id
}

pub fn get_encrypt_key() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").encrypt_key
}

pub fn github_url() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").github_url
}

pub fn doc_link() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").doc_link
}

pub fn rally_url() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").rally_url
}

pub fn workspace_id() -> &'static str {
    &CONFIG.get().expect("fail to get env variable").workspace_id
}

pub fn root_project_id() -> &'static str {
    &CONFIG
        .get()
        .expect("fail to get env variable")
        .root_project_id
}

pub fn ensure_config() {
    match ConfigEnv::new() {
        Ok(c) => {
            if let Err(e) = CONFIG.set(c) {
                eprintln!("reading env variable failed: {:?}", e);
            }
        }

        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
    println!("{}", get_db_host());
    println!("{:?}", get_db_port());
    println!("{}", get_kafka_broker_list());
    println!("{}", get_kafka_time_spent_topic());
    println!("{}", doc_link());
}
