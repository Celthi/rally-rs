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
    if let Some(obj) = obj {
        Ok(obj)
    } else {
        Err(anyhow!("No US, DE, TS found for {}. \r\n", formatted_id))
    }
}

pub async fn update_wp(ut: &UserToken, wp: &ObjectModel, body: String) -> Result<ObjectModel> {
    let url = format!(
        "{0}/{1}/{2}?key=None&workspace=workspace/{3}&project=project/{4}&projectScopeUp=false&projectScopeDown=true",
        config_env::rally_url(),
        wp.get_type(),
        wp.get_object_id(),
        config_env::workspace_id(),
        config_env::root_project_id()
    );
    let res = super::post(ut, &url, body).await?;
    let obj = res.get_object();
    if let Some(obj) = obj {
        Ok(obj)
    } else {
        Err(anyhow!(
            "No US, DE, TS found for {}. \r\n",
            wp.get_object_id()
        ))
    }
}

pub async fn set_wp_to_ready(ut: &UserToken, wp: &ObjectModel) -> Result<ObjectModel> {
    let body = format!(
        r#"{{"{0}": {{ "ObjectID": "{1}", "Ready": true}}
    }}"#,
        wp.get_type(),
        wp.get_object_id()
    );
    update_wp(ut, wp, body).await
}

#[cfg(test)]
mod test {
    use crate::config_env;

    #[test]
    fn test_set_ready() {
        let body = r#"{"FormattedID": "US1234", "Ready": true}"#;
        assert_eq!(body, r#"{"FormattedID": "US1234", "Ready": true}"#);
        // read token from env var RALLY_TOKEN
        let token = std::env::var("RALLY_TOKEN");
        let name = std::env::var("RALLY_USER");

        if token.is_err() {
            println!("Please set RALLY_TOKEN env var first.");
            return;
        }

        // create tokio runtime
        config_env::ensure_config();

        let rt = tokio::runtime::Runtime::new().unwrap();

        let ut = crate::token::tokens::UserToken {
            name: name.unwrap(),
            token: token.unwrap(),
        };
        // test set wp to ready
        let wp = rt.block_on(super::get_wp(&ut, "DE274452"));
        assert!(wp.is_ok());
        let wp = rt.block_on(super::set_wp_to_ready(&ut, &wp.unwrap()));
        assert!(wp.is_ok());
    }
}
