use crate::config_env;
use crate::github;
use crate::msg::message::TimeSpent;
use crate::rally::op::ts;
use crate::token::tokens;
use anyhow::Result;
use tracing::error;

pub async fn process(payload: &str) -> Result<()> {
    let tp: TimeSpent = serde_json::from_str(payload)?;
    match handle_time_spent(&tp).await {
        Ok(_) => {}
        Err(e) => {
            error!("processing time spent meet error: {:?}", e);
            if let (Some(repo), Some(pr)) = (tp.get_repo_name(), tp.get_pr_number()) {
                github::post_issue_comment(
                    repo,
                    pr,
                    &format!("Error: {:?}\r\n\r\n{} ", e, config_env::doc_link()),
                )
                .await?;
            }
        }
    }
    Ok(())
}
pub async fn handle_time_spent(tp: &TimeSpent) -> Result<()> {
    let login = tp.get_login_name().to_string();
    if let Ok(ut) = tokens::get_rally_token(&login).await {
        ts::add_time_spent(&ut, tp).await?;
    }
    Ok(())
}
