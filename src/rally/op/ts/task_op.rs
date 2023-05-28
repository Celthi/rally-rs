use crate::msg::message::TimeSpent;
use crate::rally::api;
use crate::rally::api::task::create_task;
use crate::rally::api::user::fetch_rally_user;
use crate::rally::models::task::CreateTask;
use crate::rally::models::{ObjectModel, Task, User};
use crate::token::tokens::UserToken;
use anyhow::Result;
use chrono::prelude::*;

fn get_task_name(date: &DateTime<Utc>) -> String {
    let s = "Code review ".to_string();
    let date = date.format("%Y-%m-%d").to_string();
    s + &date
}

/// Select a task for the owner. If the owner has a task that is not completed, then
/// return that task. Otherwise, create a new task for the owner.
pub async fn select_or_create_task(
    ut: &UserToken,
    work_product: &ObjectModel,
    tp: &TimeSpent,
) -> Result<Option<Task>> {
    let tasks = api::task::get_tasks(ut, work_product).await?;
    let owners = fetch_rally_user(ut, &ut.name).await?;
    for owner in owners.iter() {
        let t = select_task_for_owner(tasks, owner);
        if t.is_some() {
            return Ok(t);
        }

        let task_date: DateTime<Utc> = Utc::now();
        let task_name = tp
            .task_name
            .clone()
            .unwrap_or_else(|| get_task_name(&task_date));
        let ct = CreateTask::new(
            task_name,
            owner._ref.clone(),
            tp.get_time_spent(),
            work_product,
        );
        return Ok(Some(create_task(ut, &ct).await?));
    }
    Ok(None)
}

pub fn select_task_for_owner(tasks: Vec<Task>, owner: &User) -> Option<Task> {
    for t in tasks {
        if t.State != "Completed"
            && t.Owner
                .as_ref()
                .map(|o| o._refObjectUUID.as_deref())
                .flatten()
                .map(|o| o == owner._refObjectUUID)
                .unwrap_or(false)
        {
            return Some(t);
        }
    }
    None
}
