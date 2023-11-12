use serde::{Deserialize, Serialize};
pub mod project;
pub use project::Project;
mod defect;
pub use defect::Defect;
mod hr;
pub use hr::HierarchicalRequirement;
mod user;
pub use user::User;
pub mod task;
pub use task::Task;
pub mod time;
pub use time::{TimeEntryItem, TimeEntryValue};
mod testset;
pub use testset::TestSet;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PersistableObject {
    pub ObjectID: u64,
    pub _refObjectUUID: String,
    pub _ref: String,
}

/// some parent entity is collapsed into this struct
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Artifact {
    #[serde(flatten)]
    pub persistableObject: PersistableObject,
    pub Name: String,
    pub FormattedID: String,
    pub Project: Project,
    pub Ready: bool,
    pub ScheduleState: String,
    pub Owner: Option<EmbeddedObject>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmbeddedObject {
    pub _ref: String,
    pub _refObjectUUID: Option<String>,
    pub _refObjectName: Option<String>,
    pub _type: Option<String>,
}

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
#[serde(tag = "_type")] // this is the differentiator with SingleObjectModel
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
            ObjectModel::User(u) => &u.persistableObject._ref,
            ObjectModel::Defect(d) => &d.artifact.persistableObject._ref,
            ObjectModel::HierarchicalRequirement(h) => &h.artifact.persistableObject._ref,
            ObjectModel::TestSet(t) => &t.artifact.persistableObject._ref,
            ObjectModel::TimeEntryItem(t) => &t.persistableObject._ref,
            ObjectModel::Task(t) => &t.artifact.persistableObject._ref,
            ObjectModel::TimeEntryValue(t) => &t.persistableObject._ref,
        }
    }
    pub fn get_type(&self) -> &str {
        match self {
            ObjectModel::User(_) => "user",
            ObjectModel::Defect(_) => "defect",
            ObjectModel::HierarchicalRequirement(_) => "hierarchicalrequirement",
            ObjectModel::TestSet(_) => "testset",
            ObjectModel::TimeEntryItem(_) => "timeentryitem",
            ObjectModel::Task(_) => "task",
            ObjectModel::TimeEntryValue(_) => "timeentryvalue",
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
            ObjectModel::Defect(d) => d.artifact.Project.to_owned(),
            ObjectModel::HierarchicalRequirement(
                HierarchicalRequirement {
                    artifact: Artifact { Project, .. },
                    ..
                },
                ..,
            ) => Project.to_owned(),
            ObjectModel::TestSet(
                TestSet {
                    artifact: Artifact { Project, .. },
                    ..
                },
                ..,
            ) => Project.to_owned(),
            ObjectModel::TimeEntryItem(TimeEntryItem { Project, .. }) => Project.to_owned(),
            ObjectModel::Task(
                Task {
                    artifact: Artifact { Project, .. },
                    ..
                },
                ..,
            ) => Project.to_owned(),
            ObjectModel::TimeEntryValue(TimeEntryValue { .. }) => Project::default(),
        }
    }

    pub fn get_object_id(&self) -> u64 {
        match self {
            ObjectModel::User(User {
                persistableObject: PersistableObject { ObjectID, .. },
                ..
            }) => *ObjectID,
            ObjectModel::Defect(Defect {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { ObjectID, .. },
                        ..
                    },
                ..
            }) => *ObjectID,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { ObjectID, .. },
                        ..
                    },
                ..
            }) => *ObjectID,
            ObjectModel::TestSet(TestSet {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { ObjectID, .. },
                        ..
                    },
                ..
            }) => *ObjectID,
            ObjectModel::TimeEntryItem(TimeEntryItem {
                persistableObject: PersistableObject { ObjectID, .. },
                ..
            }) => *ObjectID,
            ObjectModel::Task(Task {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { ObjectID, .. },
                        ..
                    },
                ..
            }) => *ObjectID,
            ObjectModel::TimeEntryValue(TimeEntryValue {
                persistableObject: PersistableObject { ObjectID, .. },
                ..
            }) => *ObjectID,
        }
    }

    pub fn get_ref_object_uuid(&self) -> &str {
        match self {
            ObjectModel::User(User {
                persistableObject: PersistableObject { _refObjectUUID, .. },
                ..
            }) => _refObjectUUID,
            ObjectModel::Defect(Defect {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { _refObjectUUID, .. },
                        ..
                    },
                ..
            }) => _refObjectUUID,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { _refObjectUUID, .. },
                        ..
                    },
                ..
            }) => _refObjectUUID,
            ObjectModel::TestSet(TestSet {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { _refObjectUUID, .. },
                        ..
                    },
                ..
            }) => _refObjectUUID,
            ObjectModel::TimeEntryItem(TimeEntryItem {
                persistableObject: PersistableObject { _refObjectUUID, .. },
                ..
            }) => _refObjectUUID,
            ObjectModel::Task(Task {
                artifact:
                    Artifact {
                        persistableObject: PersistableObject { _refObjectUUID, .. },
                        ..
                    },
                ..
            }) => _refObjectUUID,
            ObjectModel::TimeEntryValue(TimeEntryValue {
                persistableObject: PersistableObject { _refObjectUUID, .. },
                ..
            }) => _refObjectUUID,
        }
    }

    pub fn get_formatted_id(&self) -> String {
        match self {
            ObjectModel::Defect(Defect {
                artifact: Artifact { FormattedID, .. },
                ..
            }) => FormattedID.to_owned(),
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                artifact: Artifact { FormattedID, .. },
                ..
            }) => FormattedID.to_owned(),
            ObjectModel::TestSet(TestSet {
                artifact: Artifact { FormattedID, .. },
                ..
            }) => FormattedID.to_owned(),
            ObjectModel::Task(Task {
                artifact: Artifact { FormattedID, .. },
                ..
            }) => FormattedID.to_owned(),
            _ => "no id".to_string(),
        }
    }

    pub fn get_schedule_state(&self) -> &str {
        match self {
            ObjectModel::Defect(Defect {
                artifact: Artifact { ScheduleState, .. },
                ..
            }) => ScheduleState,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                artifact: Artifact { ScheduleState, .. },
                ..
            }) => ScheduleState,
            ObjectModel::TestSet(TestSet {
                artifact: Artifact { ScheduleState, .. },
                ..
            }) => ScheduleState,
            _ => "Undefined",
        }
    }
    pub fn get_ready_status(&self) -> bool {
        match self {
            ObjectModel::Defect(Defect {
                artifact: Artifact { Ready, .. },
                ..
            }) => *Ready,
            ObjectModel::HierarchicalRequirement(HierarchicalRequirement {
                artifact: Artifact { Ready, .. },
                ..
            }) => *Ready,
            ObjectModel::TestSet(TestSet {
                artifact: Artifact { Ready, .. },
                ..
            }) => *Ready,
            _ => false,
        }
    }
}
