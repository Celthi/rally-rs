use crate::config_env;
use crate::rally::api;
use crate::rally::models;
use crate::rally::models::time::CreateItem;
use crate::rally::models::time::UpdateValue;
use crate::rally::models::ObjectModel;
use crate::token::tokens::UserToken;
use anyhow::anyhow;
use anyhow::Result;
use chrono::prelude::*;
use chrono::Duration;
use tracing::info;

pub fn get_week_start_date(date: &DateTime<Utc>) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .unwrap()
        - Duration::days((date.weekday().number_from_monday() as i64) % 7)
}
pub async fn get_time_entry_items(
    ut: &UserToken,
    start_date: &DateTime<Utc>,
    wp_id: &str,
) -> Result<Vec<models::TimeEntryItem>> {
    let start_date = get_week_start_date(start_date).format("%Y-%m-%dT%H:%M:%S.%fZ");
    let url = format!(
        r#"{0}/timeentryitem?query=(((WeekStartDate = {1}) AND (User = {2})) AND (WorkProduct.FormattedID = {3}))&fetch=true&workspace=workspace/{4}&project=project/{5}&projectScopeUp=false&projectScopeDown=true&pagesize=500&start=1"#,
        config_env::rally_url(),
        start_date,
        ut.name,
        wp_id,
        config_env::workspace_id(),
        config_env::root_project_id()
    );
    info!("{}", url);
    let res: models::RallyResult = api::get(ut, &url).await?;
    match res {
        models::RallyResult::QueryResult(models::QueryResult { Results }) => Ok(Results
            .iter()
            .filter_map(|i| match i {
                models::ObjectModel::TimeEntryItem(t) => Some(t.clone()),
                _ => None,
            })
            .collect::<Vec<models::TimeEntryItem>>()),
        _ => Err(anyhow!("No time entry item {}. \r\n", start_date)),
    }
}
pub async fn get_time_entry_values(
    ut: &UserToken,
    item_ref: &str,
) -> Result<Vec<models::TimeEntryValue>> {
    let url = format!(
        r#"{0}/timeentryvalue?workspace={0}/workspace/{1}&query=(TimeEntryItem = {item_ref})&fetch=true&start=1&pagesize=20"#,
        config_env::rally_url(),
        config_env::workspace_id()
    );
    let res: models::RallyResult = api::get(ut, &url).await?;
    match res {
        models::RallyResult::QueryResult(models::QueryResult { Results }) => Ok(Results
            .iter()
            .filter_map(|i| match i {
                models::ObjectModel::TimeEntryValue(t) => Some(t.clone()),
                _ => None,
            })
            .collect::<Vec<models::TimeEntryValue>>()),
        _ => Err(anyhow!("No time value for item {}. \r\n", item_ref)),
    }
}

pub async fn create_time_entry_item(
    ut: &UserToken,
    proj: &models::Project,
    work_project: &models::ObjectModel,
    date: &DateTime<Utc>,
    task: &models::Task,
) -> Result<models::TimeEntryItem> {
    let create_item: CreateItem = CreateItem::new(
        proj.clone(),
        api::time::get_week_start_date(date),
        work_project.clone(),
        task.clone(),
    );
    let pid = proj.get_id();
    let url = format!("{0}/timeentryitem/create?key=None&workspace=workspace/{1}&project={pid}&projectScopeUp=false&projectScopeDown=true", config_env::rally_url(), config_env::workspace_id());
    let res = api::put(ut, &url, create_item.to_json_string()).await?;
    match res.get_object() {
        Some(ObjectModel::TimeEntryItem(o)) => Ok(o),
        _ => Err(anyhow!(
            "Cannot create time entry item for user {}",
            ut.name
        )),
    }
}

pub async fn add_time_entry_value(
    ut: &UserToken,
    proj: &models::Project,
    update_value: &UpdateValue,
) -> Result<models::ObjectModel> {
    let pid = proj.get_id();

    let url = format!("{0}/timeentryvalue/create?key=None&workspace=workspace/{1}&project={pid}&projectScopeUp=false&projectScopeDown=true", config_env::rally_url(), config_env::workspace_id());
    let res = api::put(ut, &url, update_value.to_json_string()).await?;

    match res.get_object() {
        Some(o) => Ok(o),
        None => Err(anyhow!("Cannot add time spent for user {}", ut.name)),
    }
}
pub async fn update_time_entry_value(
    ut: &UserToken,
    proj: &models::Project,
    update_value: &UpdateValue,
) -> Result<models::ObjectModel> {
    let pid = proj.get_id();
    let oid = update_value.object_id.unwrap();

    let url = format!("{0}/timeentryvalue/{oid}?key=None&workspace=workspace/{1}&project={pid}&projectScopeUp=false&projectScopeDown=true", config_env::rally_url(), config_env::workspace_id());
    let res = api::post(ut, &url, update_value.to_json_string()).await?;

    match res.get_object() {
        Some(o) => Ok(o),
        None => Err(anyhow!("Cannot add time spent for user {}", ut.name)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_week_start_date() {
        assert_eq!(
            get_week_start_date(&Utc.with_ymd_and_hms(2022, 11, 16, 0, 0, 0).unwrap()),
            "2022-11-13T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );

        assert_eq!(
            get_week_start_date(&Utc.with_ymd_and_hms(2023, 2, 19, 0, 0, 0).unwrap()),
            "2023-02-19T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
    }
}
