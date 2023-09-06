mod entry;
mod task_op;
use crate::{msg::message::TimeSpent, rally::api::wp::set_wp_to_ready};
use crate::rally::api::task::update_task;
use crate::rally::api::wp::get_wp;
use crate::rally::models::ObjectModel;
use crate::token::tokens::UserToken;
use anyhow::{bail, Result};
use chrono::prelude::*;
use tracing::info;


pub async fn add_time_spent(ut: &UserToken, tp: &TimeSpent) -> Result<()> {
    let Some(wp_id) = tp.get_wp_formatted_id() else {
        if let (Some(repo), Some(pr)) = (tp.get_repo_name(), tp.get_pr_number()) {
            info!("No work product provided in the PR {}/{}", repo, pr);
        }
        bail!("The PR title is not in the correct format: DExxxxx; blablala.");
    };
    add_time_sheet(ut, &wp_id, tp).await
}

/// Add time sheet to the work product.
/// If the work product is accepted, it will not add the time sheet.
/// If the work product is not accepted, it will add the time sheet to the task.
async fn add_time_sheet(ut: &UserToken, wp_id: &str, tp: &TimeSpent) -> Result<(), anyhow::Error> {
    let work_product = get_wp(ut, wp_id).await?;
    // todo: handle feature and US without task
    let schedule_state = work_product.get_schedule_state();
    if schedule_state == "Accepted" {
        bail!("Accepted item {} cannot add task.", wp_id,);
    }
    let ready = work_product.get_ready_status();
    let task_date: DateTime<Utc> = Utc::now();
    add_to_work_product(ut, task_date, wp_id, &work_product, tp).await?;
    if ready {
        info!("The work product {} is ready.", wp_id);
        set_wp_to_ready(ut, &work_product).await?;
    }
    Ok(())
}

/// Add time sheet to the work product.
async fn add_to_work_product(
    ut: &UserToken,
    task_date: DateTime<Utc>,
    wp_id: &str,
    work_product: &ObjectModel,
    tp: &TimeSpent,
) -> Result<(), anyhow::Error> {
    let (task, item) =
        entry::get_task_and_time_entry_item(ut, task_date, wp_id, &work_product, tp).await?;
    Ok(if item.is_some() && task.is_some() {
        let task = task.unwrap();
        let item = item.unwrap();
        entry::add_time_entry_value(&item, ut, task_date, tp, work_product).await?;
        let todo = task.ToDo.map(|t| t - tp.get_time_spent()).unwrap_or(0.0);
        update_task(ut, &task, todo).await?;
    } else {
        info!("Cannot add task and time item for {wp_id}");
    })
}
