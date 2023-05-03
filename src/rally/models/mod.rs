
use serde::{Deserialize, Serialize};
pub mod project;
pub use project::Project;
mod defect;
use defect::Defect;
mod hr;
pub use hr::HierarchicalRequirement;
mod user;
use user::User;
pub mod task;
pub use task::Task;
pub mod time;
pub use time::{TimeEntryItem, TimeEntryValue};
mod testset;
use testset::TestSet;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub enum RallyResult {
    QueryResult(QueryResult),
    CreateResult(CreateResult),
    OperationResult(OperationResult),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct QueryResult {
    pub Results: Vec<ObjectModel>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct CreateResult {
    Object: ObjectModel,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct OperationResult {
    Object: ObjectModel,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "_type")]
pub enum ObjectModel {
    User(User),
    HierarchicalRequirement(HierarchicalRequirement),
    Defect(Defect),
    Task(Task),
    TestSet(TestSet),
    TimeEntryItem(TimeEntryItem),
    TimeEntryValue(TimeEntryValue),
}
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SingleObjectModel {
    User(User),
    HierarchicalRequirement(HierarchicalRequirement),
    Defect(Defect),
    Task(Task),
    TestSet(TestSet),
    TimeEntryItem(TimeEntryItem),
    TimeEntryValue(TimeEntryValue),
}



impl RallyResult {
    pub fn get_object(&self) -> Option<ObjectModel> {
        match self {
            RallyResult::CreateResult(c) => Some(c.Object.clone()),
            RallyResult::QueryResult(q) => {
                if q.Results.is_empty() {
                    None
                } else {
                    Some(q.Results[0].clone())
                }
            }
            RallyResult::OperationResult(c) => Some(c.Object.clone()),
        }
    }
}
impl ObjectModel {
    pub fn get_ref(&self) -> &str {
        match self {
            ObjectModel::User(u) => &u._ref,
            ObjectModel::Defect(d) => &d._ref,
            ObjectModel::HierarchicalRequirement(h) => &h._ref,
            ObjectModel::TestSet(t) => &t._ref,
            ObjectModel::TimeEntryItem(t) => &t._ref,
            ObjectModel::Task(t) => &t._ref,
            ObjectModel::TimeEntryValue(t) => &t._ref,
        }
    }
    pub fn get_project(&self) -> Project {
        match self {
            ObjectModel::User(u) => {
                if u.DefaultProject.is_some() {
                    u.DefaultProject.to_owned().unwrap()
                } else {
                    Project::default()
                }
            }
            ObjectModel::Defect(d) => d.Project.to_owned(),
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement { Project, .. }) => {
                Project.to_owned()
            }
            ObjectModel::TestSet(TestSet { Project, .. }) => Project.to_owned(),
            ObjectModel::TimeEntryItem(TimeEntryItem { Project, .. }) => Project.to_owned(),
            ObjectModel::Task(Task { Project, .. }) => Project.to_owned(),
            ObjectModel::TimeEntryValue(TimeEntryValue { .. }) => Project::default(),
        }
    }
    pub fn get_object_id(&self) -> u64 {
        match self {
            ObjectModel::User(User { ObjectID, .. }) => *ObjectID,
            ObjectModel::Defect(Defect { ObjectID, .. }) => *ObjectID,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement { ObjectID, .. }) => {
                *ObjectID
            }
            ObjectModel::TestSet(TestSet { ObjectID, .. }) => *ObjectID,
            ObjectModel::TimeEntryItem(TimeEntryItem { ObjectID, .. }) => *ObjectID,
            ObjectModel::Task(Task { ObjectID, .. }) => *ObjectID,
            ObjectModel::TimeEntryValue(TimeEntryValue { ObjectID, .. }) => *ObjectID,
        }
    }
    pub fn get_ref_object_uuid(&self) -> &str {
        match self {
            ObjectModel::User(User { _refObjectUUID, .. }) => _refObjectUUID,
            ObjectModel::Defect(Defect { _refObjectUUID, .. }) => _refObjectUUID,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                _refObjectUUID,
                ..
            }) => _refObjectUUID,
            ObjectModel::TestSet(TestSet { _refObjectUUID, .. }) => _refObjectUUID,
            ObjectModel::TimeEntryItem(TimeEntryItem { _refObjectUUID, .. }) => _refObjectUUID,
            ObjectModel::Task(Task { _refObjectUUID, .. }) => _refObjectUUID,
            ObjectModel::TimeEntryValue(TimeEntryValue { _refObjectUUID, .. }) => _refObjectUUID,
        }
    }
    pub fn get_formatted_id(&self) -> String {
        match self {
            ObjectModel::Defect(Defect { FormattedID, .. }) => FormattedID.to_owned(),
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                FormattedID, ..
            }) => FormattedID.to_owned(),
            ObjectModel::TestSet(TestSet { FormattedID, .. }) => FormattedID.to_owned(),
            ObjectModel::Task(Task { FormattedID, .. }) => FormattedID.to_owned(),
            _ => "no id".to_string(),
        }
    }
    pub fn get_schedule_state(&self) -> &str {
        match self {
            ObjectModel::Defect(Defect { ScheduleState, .. }) => ScheduleState,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                ScheduleState, ..
            }) => ScheduleState,
            ObjectModel::TestSet(TestSet { ScheduleState, .. }) => ScheduleState,
            _ => "Undefined",
        }
    }
}
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbeddedObject {
    pub _ref: String,
    pub _refObjectUUID: Option<String>,
    pub _refObjectName: Option<String>,
    pub _type: Option<String>,
}
