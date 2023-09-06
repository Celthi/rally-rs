use super::*;
use crate::rally::models;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeEntryItem {
    pub Project: Project,
    pub WorkProduct: EmbeddedObject,
    pub _ref: String,
    pub ObjectID: u64,
    pub _refObjectUUID: String,
    pub Task: EmbeddedObject,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TimeEntryValue {
    pub _ref: String,
    pub TimeEntryItem: EmbeddedObject,
    pub ObjectID: u64,
    pub _refObjectUUID: String,
    pub DateVal: DateTime<Utc>,
    pub Hours: f32,
}

pub struct CreateItem<'a, 'b, 'c> {
    project: &'a models::Project,
    week_start_date: DateTime<Utc>,
    work_product: &'b models::ObjectModel,
    task: &'c models::Task,
}

impl<'a, 'b, 'c> CreateItem<'a, 'b, 'c> {
    pub fn new(
        project: &'a models::Project,
        week_start_date: DateTime<Utc>,
        work_product: &'b models::ObjectModel,
        task: &'c models::Task,
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

pub struct UpdateValue<'a> {
    date_val: DateTime<Utc>,
    hours: f32,
    item_ref: &'a str,
    pub object_id: Option<u64>,
}

impl<'a> UpdateValue<'a> {
    pub fn new(date_val: DateTime<Utc>, hours: f32, item_ref: &'a str) -> Self {
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

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        use super::*;
        let s = r#"{
            "_ref": "/timeentryvalue/699271222795",
            "_refObjectUUID": "db96b167-000a-418b-8ca6-927044d86c36",
            "_p": "7",
            "ObjectID": 699271222795,
            "DateVal": "2023-05-03T00:00:00.000Z",
            "Hours": 1.0,
            "TimeEntryItem": {
                "_ref": "/timeentryitem/699256908135"
            },
            "_type": "TimeEntryValue"
        }"#;
        let _t: TimeEntryValue = serde_json::from_str(s).unwrap();
    }
}
