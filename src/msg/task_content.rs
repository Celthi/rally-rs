
use std::collections::HashMap;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tracing::info;
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct TaskContentMap {
    pub map: HashMap<String, HashMap<String, String>>,
}


// create a global instance of the map
static TASK_CONTENT_MAP: OnceCell<TaskContentMap> = OnceCell::new();

impl TaskContentMap {
    pub fn get_task_content(&self, source: &str, text: &str) -> Option<String> {
        info!("source: {}, text: {}", source, text);
        let text = text.to_lowercase();
        let map = self.map.get(source)?;
        for (key, value) in map {
            if text.contains(key) {
                return Some(value.to_string());
            }
        }
        None
    }
    /// read the map from json file task_content_map.json
    pub fn new() -> Self {
        let map = serde_json::from_str::<TaskContentMap>(include_str!("task_content_map.json")).expect("There should be a task_content_map.json file in the current directory");
        map
    }
    pub fn global() -> &'static TaskContentMap {
        TASK_CONTENT_MAP.get().expect("task content to be initialized")
    }
}

