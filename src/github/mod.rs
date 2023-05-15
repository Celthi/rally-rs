use crate::config_env;
use anyhow::Result;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostComment<'a> {
    body: &'a str,
}
pub async fn post_issue_comment(repo_name: &str, pr_number: u64, s: &str) -> Result<()> {
    post_issue_comment_internal(
        config_env::github_url(),
        config_env::get_github_token(),
        repo_name,
        pr_number,
        s,
    )
    .await
}

async fn post_issue_comment_internal(
    github_api_url: &str,
    token: &str,
    repo_name: &str,
    pr_number: u64,
    s: &str,
) -> Result<()> {
    let comment_url = format!("{github_api_url}/repos/{repo_name}/issues/{pr_number}/comments");

    let client = reqwest::Client::new();
    let data = &PostComment { body: s };

    client
        .post(comment_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "Request")
        .json(data)
        .send()
        .await?;
    Ok(())
}
