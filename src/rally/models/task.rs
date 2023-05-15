use super::*;
use crate::rally::models;
use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub Name: String,
    pub FormattedID: String,
    pub Project: Project,
    pub Ready: bool,
    pub Estimate: Option<f32>,
    pub State: String,
    pub TimeSpent: f32,
    pub ToDo: f32,
    pub _ref: String,
    pub ObjectID: u64,
    pub Owner: Option<EmbeddedObject>,
    pub _refObjectUUID: String,
}

#[derive(Clone)]
pub struct CreateTask<'a> {
    name: String,
    owner: String,
    estimate: f32,
    work_product: &'a models::ObjectModel,
}

impl<'a> CreateTask<'a> {
    pub fn new(
        name: String,
        owner: String,
        estimate: f32,
        work_product: &'a models::ObjectModel,
    ) -> Self {
        CreateTask {
            name,
            owner,
            estimate,
            work_product,
        }
    }
    pub fn to_json_string(&self) -> String {
        format!(
            r#"
        {{
                "Task": {{
                    "Name": "{0}",
                    "Owner": "{1}",
                    "Estimate": {2},
                    "State": "Defined",
                    "WorkProduct": "{3}"
                }}
            }}
        "#,
            self.name,
            self.owner,
            self.estimate,
            self.work_product.get_ref()
        )
    }
}

pub struct UpdateTask {
    state: String,
    formatted_id: String,
    to_do: f32,
    object_id: u64,
}

impl UpdateTask {
    pub fn new(state: String, formatted_id: String, to_do: f32, object_id: u64) -> Self {
        UpdateTask {
            state,
            formatted_id,
            to_do,
            object_id,
        }
    }
    pub fn to_json_string(&self) -> String {
        format!(
            r#"
        {{
                "Task": {{
                    "State": "{0}",
                    "FormattedID": "{1}",
                    "ToDo": {2},
                    "ObjectID": {3}
                }}
            }}
        "#,
            self.state, self.formatted_id, self.to_do, self.object_id
        )
    }
}
