use super::task_op::select_or_create_task;
use crate::msg::message::TimeSpent;
use crate::rally::api;
use crate::rally::models::time::UpdateValue;
use crate::rally::models::ObjectModel;
use crate::rally::models::SingleObjectModel;
use crate::rally::models::Task;
use crate::token::tokens::UserToken;
use anyhow::Result;
use chrono::prelude::*;
use tracing::error;

use crate::rally::models::TimeEntryItem;

pub async fn add_time_entry_value(
    item: &TimeEntryItem,
    ut: &UserToken,
    task_date: DateTime<Utc>,
    tp: &TimeSpent,
    work_product: &ObjectModel,
) -> Result<(), anyhow::Error> {
    let update_value = create_update_value(item, ut, task_date, tp).await?;
    if update_value.object_id.is_none() {
        api::time::add_time_entry_value(ut, &work_product.get_project(), &update_value).await?;
    } else {
        api::time::update_time_entry_value(ut, &work_product.get_project(), &update_value).await?;
    }
    Ok(())
}

pub async fn create_update_value<'a>(
    item: &'a TimeEntryItem,
    ut: &UserToken,
    task_date: DateTime<Utc>,
    tp: &TimeSpent,
) -> Result<UpdateValue<'a>, anyhow::Error> {
    let values = api::time::get_time_entry_values(ut, &item.persistableObject._ref).await?;
    let mut update_value = UpdateValue::new(task_date, tp.get_time_spent(), &item.persistableObject._ref);
    values.iter().for_each(|i| {
        if i.DateVal.year() == task_date.year()
            && i.DateVal.month() == task_date.month()
            && i.DateVal.day() == task_date.day()
        {
            update_value.set_object_id(i.persistableObject.ObjectID);
            update_value.add_hours(i.Hours);
        }
    });
    Ok(update_value)
}

pub async fn get_uncompleted_task_and_entry_item(
    ut: &UserToken,
    tis: Vec<TimeEntryItem>,
) -> Result<Option<(Task, TimeEntryItem)>> {
    for i in tis {
        if let SingleObjectModel::Task(t) =
            api::fetch_object::<SingleObjectModel>(ut, &i.Task._ref).await?
        {
            if t.State != "Completed" {
                return Ok(Some((t, i)));
            }
        }
    }
    Ok(None)
}
/// get task and time entry item
/// if there is a uncompleted task, return it
/// if there is no uncompleted task, create a new task and time entry item
/// if there is no task, return None
pub async fn get_task_and_time_entry_item(
    ut: &UserToken,
    task_date: DateTime<Utc>,
    wp_id: &str,
    work_product: &ObjectModel,
    tp: &TimeSpent,
) -> Result<(Option<Task>, Option<TimeEntryItem>)> {
    let tis = api::time::get_time_entry_items(ut, &task_date, wp_id).await?;
    let uncompleted_task_and_entry_item = get_uncompleted_task_and_entry_item(ut, tis).await?;
    if let Some((task, item)) = uncompleted_task_and_entry_item {
        return Ok((Some(task), Some(item)));
    }
    if let Some(task) = select_or_create_task(ut, work_product, tp).await? {
        let item = Some(
            api::time::create_time_entry_item(
                ut,
                &work_product.get_project(),
                work_product,
                &task_date,
                &task,
            )
            .await?,
        );
        return Ok((Some(task), item));
    }
    error!("cannot find task end item for {}", wp_id);
    Ok((None, None))
}
