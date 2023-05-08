use crate::config_env;
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostComment<'a> {
    body: &'a str,
}

pub async fn post_issue_comment(repo_name: &str, pr_number: u64, s: &str) -> Result<()> {
    let comment_url = format!(
        "{0}/api/v3/repos/{1}/issues/{2}/comments",
        config_env::github_url(),
        repo_name,
        pr_number
    );

    let client = reqwest::Client::new();
    let data = &PostComment { body: s };

    client
        .post(comment_url)
        .header(
            "Authorization",
            format!("token {}", config_env::get_github_token()),
        )
        .header("Accept", "application/vnd.github+json")
        .json(data)
        .send()
        .await?;
    Ok(())
}
