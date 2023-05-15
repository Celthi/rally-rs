use crate::config_env;
use crate::rally::models::RallyResult;
use crate::token::tokens::UserToken;

use crate::rally::api;
use crate::rally::models::ObjectModel;

use anyhow::{anyhow, Result};
// name is the email address
pub async fn fetch_rally_user(ut: &UserToken, name: &str) -> Result<ObjectModel> {
    let url = format!("{0}/User?fetch=true&query=(UserName%20%3D%20%22{name}%22)&workspace=workspace/{1}&project=project/{2}&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1",
    config_env::rally_url(),
    config_env::workspace_id(),
    config_env::root_project_id());
    let res = api::get::<RallyResult>(ut, &url).await?;
    let object = res.get_object();
    match object {
        Some(o) => Ok(o),
        None => Err(anyhow!(format!(
            "No Rally user for {} or the Rally token is invalid.\r\n",
            ut.name
        ))),
    }
}
