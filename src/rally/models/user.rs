use super::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub c_EmployeeID: Option<String>,
    pub UserName: String,
    pub DefaultProject: Option<Project>,
    pub _ref: String,
    pub ObjectID: u64,
    pub _refObjectUUID: String,
}
