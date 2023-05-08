use super::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestSet {
    pub Name: String,
    pub FormattedID: String,
    pub Project: Project,
    pub Ready: bool,
    pub _ref: String,
    pub ObjectID: u64,
    pub ScheduleState: String,
    pub _refObjectUUID: String,
}
