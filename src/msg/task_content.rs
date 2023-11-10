use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct TaskContentMap {
    #[serde(flatten)]
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
    pub fn new() {
        let s = include_str!("task_content_map.json");
        let map = serde_json::from_str::<TaskContentMap>(s)
            .expect("There should be a task_content_map.json file in the current directory");
        TASK_CONTENT_MAP
            .set(map)
            .expect("task content map to be initialized");
    }
    pub fn global() -> &'static TaskContentMap {
        TASK_CONTENT_MAP
            .get()
            .expect("task content to be initialized")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // #[test]
    // fn test_get_task_content() {
    //     TaskContentMap::new();
    //     let source = "github";
    //     let text = "thanks";
    //     let task_name = TaskContentMap::global().get_task_content(source, text);
    //     assert_eq!(task_name, Some("Code Review".to_string()));
    // }
    #[test]
    fn test_global() {
        TaskContentMap::new();
        let map = TaskContentMap::global();
        let source = "rally";
        let text = "update";
        let task_name = map.get_task_content(source, text);
        assert_eq!(task_name, Some("Investigate the issue".to_string()));
        let source = "github";
        let text = "thanks";
        let task_name = TaskContentMap::global().get_task_content(source, text);
        assert_eq!(task_name, Some("Code Review".to_string()));
    }
}
