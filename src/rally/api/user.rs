use crate::config_env;
use crate::rally::api;
use crate::rally::models::ObjectModel;
use crate::rally::models::{RallyResult, User};
use crate::token::tokens::UserToken;
use anyhow::{anyhow, Result};

// name is the email address
pub async fn fetch_rally_user(ut: &UserToken, name: &str) -> Result<Vec<User>> {
    let url = format!("{0}/User?fetch=true&query=(UserName%20%3D%20%22{name}%22)&workspace=workspace/{1}&project=project/{2}&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1",
    config_env::rally_url(),
    config_env::workspace_id(),
    config_env::root_project_id());
    let res = api::get::<RallyResult>(ut, &url).await?;
    match res {
        RallyResult::QueryResult(qr) => {
            let results = qr.Results;
            Ok(results.into_iter().filter_map(|i| match i {
                ObjectModel::User(u) => Some(u),
                _ => None,
            }).collect::<Vec<User>>())
        }
        _ => Err(anyhow!(format!(
            "No Rally user for {} or the Rally token is invalid.\r\n",
            name
        ))),
    }

}
