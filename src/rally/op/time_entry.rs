use crate::rally::models;
use chrono::prelude::*;

pub struct CreateItem {
    project: models::Project,
    week_start_date: DateTime<Utc>,
    work_product: models::ObjectModel,
    task: models::Task,
}

impl CreateItem {
    pub fn new(
        project: models::Project,
        week_start_date: DateTime<Utc>,
        work_product: models::ObjectModel,
        task: models::Task,
    ) -> Self {
        CreateItem {
            project,
            week_start_date,
            work_product,
            task,
        }
    }
    pub fn to_json_string(&self) -> String {
        format!(
            r#"
        {{
            "TimeEntryItem": {{
                "Project": "{0}",
                "WeekStartDate": "{1}",
                "WorkProduct": "{2}",
                "Task": "{3}"
            }}
        }}
        "#,
            self.project.get_id(),
            self.week_start_date.format("%Y-%m-%dT%H:%M:%S.%fZ"),
            self.work_product.get_ref(),
            self.task._ref
        )
    }
}

pub struct UpdateValue {
    date_val: DateTime<Utc>,
    hours: f32,
    item_ref: String,
    pub object_id: Option<u64>,
}

impl UpdateValue {
    pub fn new(date_val: DateTime<Utc>, hours: f32, item_ref: String) -> Self {
        UpdateValue {
            date_val,
            hours,
            item_ref,
            object_id: None,
        }
    }
    pub fn set_object_id(&mut self, oid: u64) {
        self.object_id = Some(oid);
    }
    pub fn add_hours(&mut self, hours: f32) {
        self.hours += hours;
    }
    pub fn to_json_string(&self) -> String {
        if self.object_id.is_none() {
            format!(
                r#"
            {{
                "TimeEntryValue": {{
                    "DateVal": "{0}",
                    "Hours": {1},
                    "TimeEntryItem": "{2}"
                }}
            }}"#,
                self.date_val.format("%Y-%m-%dT%H:%M:%S.%fZ"),
                self.hours,
                self.item_ref
            )
        } else {
            format!(
                r#"
                {{
                    "TimeEntryValue": {{
                        "DateVal": "{0}",
                        "Hours": {1},
                        "TimeEntryItem": "{2}",
                        "ObjectID": {3}
                    }}
                }}"#,
                self.date_val.format("%Y-%m-%dT%H:%M:%S.%fZ"),
                self.hours,
                self.item_ref,
                self.object_id.unwrap()
            )
        }
    }
}
