use crate::msg::message::TimeSpent;
use crate::rally::api::task::{create_task, update_task};
use crate::rally::api::user::fetch_rally_user;
use crate::rally::api::wp::get_wp;
use crate::rally::models;
use crate::rally::models::task::CreateTask;
use crate::rally::models::time::UpdateValue;
use crate::rally::{api, models::SingleObjectModel};
use crate::token::tokens::UserToken;
use anyhow::{bail, Result};
use chrono::prelude::*;

use tracing::info;

fn get_task_name(date: &DateTime<Utc>) -> String {
    let s = "Code review ".to_string();
    let date = date.format("%Y-%m-%d").to_string();
    s + &date
}

pub async fn add_time_spent(ut: &UserToken, tp: &TimeSpent) -> Result<()> {
    match tp.get_wp_formatted_id() {
        Some(wp_id) => {
            add_time_sheet(ut, &wp_id, tp).await?;
        }
        None => {
            if let (Some(repo), Some(pr)) = (tp.get_repo_name(), tp.get_pr_number()) {
                info!("No work product provided in the PR {}{}", repo, pr);
            }
            bail!("The PR title is not in the correct format: DExxxxx; blablala.")
        }
    }
    Ok(())
}

async fn add_time_sheet(ut: &UserToken, wp_id: &str, tp: &TimeSpent) -> Result<(), anyhow::Error> {
    let work_product = get_wp(ut, wp_id).await?;
    let schedule_state = work_product.get_schedule_state();
    if schedule_state == "Accepted" {
        bail!("Accepted item {} cannot add task.", wp_id,);
    }
    let task_date: DateTime<Utc> = Utc::now();
    let (task, item) =
        get_task_and_time_entry_item(ut, task_date, wp_id, &work_product, tp).await?;
    if item.is_some() && task.is_some() {
        let task = task.unwrap();
        let item = item.unwrap();
        add_time_entry_value(&item, ut, task_date, tp, work_product).await?;
        let todo = task.ToDo - tp.get_time_spent();
        update_task(ut, &task, todo).await?;
    }

    Ok(())
}

async fn add_time_entry_value(
    item: &models::TimeEntryItem,
    ut: &UserToken,
    task_date: DateTime<Utc>,
    tp: &TimeSpent,
    work_product: models::ObjectModel,
) -> Result<(), anyhow::Error> {
    let update_value = create_update_value(item, ut, task_date, tp).await?;
    if update_value.object_id.is_none() {
        api::time::add_time_entry_value(ut, &work_product.get_project(), &update_value).await?;
    } else {
        api::time::update_time_entry_value(ut, &work_product.get_project(), &update_value).await?;
    }
    Ok(())
}

async fn create_update_value(
    item: &models::TimeEntryItem,
    ut: &UserToken,
    task_date: DateTime<Utc>,
    tp: &TimeSpent,
) -> Result<UpdateValue, anyhow::Error> {
    let values = api::time::get_time_entry_values(ut, &item._ref).await?;
    let mut update_value = UpdateValue::new(task_date, tp.get_time_spent(), item._ref.clone());
    values.iter().for_each(|i| {
        if i.DateVal.year() == task_date.year()
            && i.DateVal.month() == task_date.month()
            && i.DateVal.day() == task_date.day()
        {
            update_value.set_object_id(i.ObjectID);
            update_value.add_hours(i.Hours);
        }
    });
    Ok(update_value)
}

async fn get_task_and_time_entry_item(
    ut: &UserToken,
    task_date: DateTime<Utc>,
    wp_id: &str,
    work_product: &models::ObjectModel,
    tp: &TimeSpent,
) -> Result<(Option<models::Task>, Option<models::TimeEntryItem>)> {
    let tis = api::time::get_time_entry_items(ut, &task_date, wp_id).await?;
    let mut task = None;
    let mut item = None;
    if !tis.is_empty() {
        for i in tis {
            let res = api::fetch_object::<SingleObjectModel>(ut, &i.Task._ref).await?;
            if let SingleObjectModel::Task(t) = res {
                if t.State != "Completed" {
                    task = Some(t);
                    item = Some(i);
                }
            }
        }
    }
    if item.is_none() {
        task = select_task(ut, work_product, tp).await?;
        if task.is_some() {
            item = Some(
                api::time::create_time_entry_item(
                    ut,
                    &work_product.get_project(),
                    work_product,
                    &task_date,
                    task.as_ref().unwrap(),
                )
                .await?,
            );
        }
    }
    Ok((task, item))
}

async fn select_task(
    ut: &UserToken,
    work_product: &models::ObjectModel,
    tp: &TimeSpent,
) -> Result<Option<models::Task>> {
    let tasks = api::task::get_tasks(ut, work_product).await?;
    let owner = fetch_rally_user(ut, &ut.name).await?;
    let mut task = None;
    for t in tasks {
        if t.State != "Completed"
            && t.Owner._refObjectUUID.is_some()
            && t.Owner._refObjectUUID.as_deref().unwrap() == owner.get_ref_object_uuid()
        {
            task = Some(t);
        }
    }
    if task.is_none() {
        let task_date: DateTime<Utc> = Utc::now();
        let task_name = tp
            .task_name
            .clone()
            .unwrap_or_else(|| get_task_name(&task_date));
        let ct = CreateTask::new(
            task_name,
            owner.get_ref().to_string(),
            tp.get_time_spent(),
            work_product,
        );
        task = Some(create_task(ut, &ct).await?);
    }
    Ok(task)
}
