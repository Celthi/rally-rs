use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Project {
    pub _ref: String,
    pub _refObjectName: String,
    pub _refObjectUUID: String,

}

impl Project {
    pub fn get_id(&self) -> String {
        self._ref.chars().skip(48).collect::<String>()
    }
}

impl Default for Project {
    fn default() -> Self {
        Project {
            _ref: "{0}/project/27956542317".to_string(),
            _refObjectName: "Tec".to_string(),
            _refObjectUUID: "hi".to_string(),
        }
    }
}
