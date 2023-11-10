use crate::msg::message::TimeSpent;
use crate::rally::api;
use crate::rally::api::task::create_task;
use crate::rally::api::user::fetch_rally_user;
use crate::rally::models::task::CreateTask;
use crate::rally::models::{ObjectModel, Task, User};
use crate::token::tokens::UserToken;
use anyhow::Result;
use tracing::info;


/// Select a task for the owner. If the owner has a task that is not completed, then
/// return that task. Otherwise, create a new task for the owner.
pub async fn select_or_create_task(
    ut: &UserToken,
    work_product: &ObjectModel,
    tp: &TimeSpent,
) -> Result<Option<Task>> {
    let tasks = api::task::get_tasks(ut, work_product).await?;
    let owners = fetch_rally_user(ut, &ut.name).await?;
    let owner = owners.first().ok_or(anyhow::anyhow!("No owner found"))?;
    let t = select_task_for_owner(&tasks, owner);
    if t.is_some() {
        return Ok(t.cloned());
    }
    info!("Creating task for {}", tp.get_user_name());
    // if we get here, then we need to create a new task
    let task_name = tp
        .task_name
        .clone()
        .unwrap_or_else(|| tp.get_task_name());
    let ct = CreateTask::new(
        task_name,
        owner._ref.clone(),
        tp.get_time_spent(),
        work_product,
    );
    Ok(Some(create_task(ut, &ct).await?))
}

pub fn select_task_for_owner<'a>(tasks: &'a [Task], owner: &User) -> Option<&'a Task> {
    tasks.iter().find(|t| {
        let is_owned = owned_by_user(t, owner);
        t.State != "Completed" && is_owned
    })
}

fn owned_by_user(t: &Task, owner: &User) -> bool {
    t.Owner
        .as_ref()
        .and_then(|o| o._refObjectUUID.as_deref())
        .map(|uuid| uuid == owner._refObjectUUID)
        .unwrap_or(false)
}
