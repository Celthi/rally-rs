use crate::config_env;
use crate::rally::api;
use crate::rally::models;
use crate::rally::models::task::CreateTask;
use crate::rally::models::task::UpdateTask;
use crate::token::tokens::UserToken;
use anyhow::anyhow;
use anyhow::Result;
pub async fn create_task<'a, 'b>(
    ut: &UserToken,
    create_task: &'b CreateTask<'a>,
) -> Result<models::Task> {
    let url = format!("{}/task/create?key=None&workspace=workspace/27397600726&project=project/40120756498&projectScopeUp=false&projectScopeDown=true", config_env::rally_url());
    let res = api::post(ut, &url, create_task.to_json_string()).await?;
    match res.get_object() {
        Some(models::ObjectModel::Task(t)) => Ok(t),
        _ => Err(anyhow!(
            "Cannot create task {}",
            create_task.to_json_string()
        )),
    }
}

pub async fn update_task(
    ut: &UserToken,
    task: &models::Task,
    todo: f32,
) -> Result<models::ObjectModel> {
    let oid = task.ObjectID;
    let url = format!("{0}/task/{oid}?key=None&workspace=workspace/27397600726&project=project/40120756498&projectScopeUp=false&projectScopeDown=true", config_env::rally_url());
    let mut state = "In-Progress".to_string();
    if todo <= 0f32 {
        state = "Completed".to_string();
    }
    let update_task_value = UpdateTask::new(state, task.FormattedID.clone(), todo, task.ObjectID);
    let res = api::post(ut, &url, update_task_value.to_json_string()).await?;
    match res.get_object() {
        Some(o) => Ok(o),
        None => Err(anyhow!("No task found {}. \r\n", oid)),
    }
}

pub async fn get_tasks(ut: &UserToken, wp: &models::ObjectModel) -> Result<Vec<models::Task>> {
    match wp {
        models::ObjectModel::HierarchicalRequirement(models::HierarchicalRequirement {
            Tasks,
            ..
        }) => {
            if Tasks.is_some() {
                if let models::RallyResult::QueryResult(q) =
                    api::get(ut, &Tasks.as_ref().unwrap()._ref).await?
                {
                    return Ok(q
                        .Results
                        .iter()
                        .filter_map(|i| match i {
                            models::ObjectModel::Task(t) => Some(t.clone()),
                            _ => None,
                        })
                        .collect::<Vec<models::Task>>());
                }
            }
            Ok(vec![])
        }
        _ => Ok(vec![]),
    }
}
