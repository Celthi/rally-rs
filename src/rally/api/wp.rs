use crate::config_env;
use crate::rally::models::{ObjectModel, RallyResult};
use crate::token::tokens::UserToken;
use anyhow::{anyhow, Result};

// work product: US, DE, TS, etc.
pub async fn get_wp(ut: &UserToken, formatted_id: &str) -> Result<ObjectModel> {
    let query_part = format!("fetch=true&query=(FormattedID%20%3D%20%22{formatted_id}%22)&workspace=workspace/{0}&project=project/{1}&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1", 
    config_env::workspace_id(),
    config_env::root_project_id()
);
    let url = if formatted_id.starts_with("DE") {
        format!("{0}/Defect?{1}", config_env::rally_url(), query_part)
    } else if formatted_id.starts_with("US") {
        format!(
            "{0}/HierarchicalRequirement?{1}",
            config_env::rally_url(),
            query_part
        )
    } else if formatted_id.starts_with("TS") {
        format!("{0}/TestSet?{1}", config_env::rally_url(), query_part)
    } else {
        "".to_string()
    };

    let res = super::get::<RallyResult>(ut, &url).await?;
    let obj = res.get_object();
    if obj.is_some() {
        Ok(obj.unwrap())
    } else {
        Err(anyhow!("No US, DE, TS found for {}. \r\n", formatted_id))
    }
}
