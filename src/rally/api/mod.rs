use crate::rally::models::RallyResult;
use crate::token::tokens::UserToken;
use anyhow::{anyhow, Result};
use reqwest;
use reqwest::Response;
use serde::de::DeserializeOwned;
use tracing::error;
pub mod task;
pub mod time;
pub mod user;
pub mod wp;
async fn get<T:DeserializeOwned>(ut: &UserToken, url: &str) -> Result<T> {
    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("ZSESSIONID", &ut.token)
        .send()
        .await?;
    get_results(resp).await
}

async fn post<'a, 'b>(ut: &'a UserToken, url: &'b str, body: String) -> Result<RallyResult> {
    let client = reqwest::Client::new();

    let resp = client
        .post(url)
        .header("ZSESSIONID", &ut.token)
        .body(body)
        .send()
        .await?;

    get_results(resp).await
}

async fn put<'a, 'b>(ut: &'a UserToken, url: &'b str, body: String) -> Result<RallyResult> {
    let client = reqwest::Client::new();

    let resp = client
        .put(url)
        .header("ZSESSIONID", &ut.token)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    get_results(resp).await
}

async fn get_results<T: DeserializeOwned>(resp: Response) -> Result<T>
{
    let status = resp.status();
    let text = resp.text().await?;
    if status.is_success() {
        match serde_json::from_str::<T>(&text) {
            Ok(o) => Ok(o),
            Err(e) => {
                error!(
                    "cannot convert the Rally response to the object model: {:?}, text: {}",
                    e, text
                );
                Err(anyhow!("{:?}. {}", e, text))
            }
        }
    } else {
        error!("fetch response from Rally meet error: {}", text);
        Err(anyhow!(format!("Error while geting response from the Rally. Possible reason: 1. Rally server is down. 2 Your Rally API token is invalid. \r\n\r\n. Rally Response is: {}", text)))
    }
}

pub async fn fetch_object<T:DeserializeOwned>(ut: &UserToken, _ref: &str) -> Result<T> {
    let url = format!("{_ref}");
    get::<T>(ut, &url).await
}
