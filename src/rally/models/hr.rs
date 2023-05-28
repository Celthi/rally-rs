use super::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HierarchicalRequirement {
    pub Name: String,
    pub FormattedID: String,
    pub Project: Project,
    pub Ready: bool,
    pub _ref: String,
    pub ObjectID: u64,
    pub ScheduleState: String,
    pub Tasks: Option<EmbeddedObject>,
    pub _refObjectUUID: String,
}
