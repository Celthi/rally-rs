use crate::config_env;
use crate::rally::models::{ObjectModel, RallyResult};
use crate::token::tokens::UserToken;
use anyhow::{anyhow, Result};

// work product: US, DE, TS, etc.
pub async fn get_wp(ut: &UserToken, formatted_id: &str) -> Result<ObjectModel> {
    let url;
    if formatted_id.starts_with("DE") {
        url = format!("{0}/Defect?fetch=true&query=(FormattedID%20%3D%20%22{formatted_id}%22)&workspace=workspace/27397600726&project=project/40120756498&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1", config_env::rally_url());
    } else if formatted_id.starts_with("US") {
        url = format!("{0}/HierarchicalRequirement?fetch=true&query=(FormattedID%20%3D%20%22{formatted_id}%22)&workspace=workspace/27397600726&project=project/40120756498&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1", config_env::rally_url());
    } else if formatted_id.starts_with("TS") {
        url = format!("{0}/TestSet?fetch=true&query=(FormattedID%20%3D%20%22{formatted_id}%22)&workspace=workspace/27397600726&project=project/40120756498&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1", config_env::rally_url());
    } else {
        url = "".to_string();
    }

    let res = super::get::<RallyResult>(ut, &url).await?;
    let obj = res.get_object();
    if obj.is_some() {
        Ok(obj.unwrap())
    } else {
        Err(anyhow!("No US, DE, TS found for {}. \r\n", formatted_id))
    }
}
