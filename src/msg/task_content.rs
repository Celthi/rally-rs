use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct TaskContentMap {
    #[serde(flatten)]
    pub map: HashMap<String, HashMap<String, String>>,
}
impl TaskContentMap {
    pub fn from_file() -> Self {
        let s = std::fs::read_to_string("task_content_map.json")
            .expect("There should be a task_content_map.json file in the current directory");
        serde_json::from_str::<TaskContentMap>(&s)
            .expect("There should be a task_content_map.json file in the current directory")
    }
}

fn global_task_content_map() -> &'static TaskContentMap {
    static INSTANCE: OnceCell<TaskContentMap> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        TaskContentMap::from_file()
    })
}


impl TaskContentMap {
    fn get_task_content(&self, source: &str, text: &str) -> Option<String> {
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

    pub fn get_task_name(source: &str, text: &str) -> String {
        if let Some(value) = global_task_content_map().get_task_content(source, text) {
            return value;
        }
        "undefined".to_string()
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
        let source = "rally";
        let text = "update";
        let task_name = TaskContentMap::get_task_name(source, text);
        assert_eq!(task_name, "Investigate the issue".to_string());
        let source = "github";
        let text = "thanks";
        let task_name = TaskContentMap::get_task_name(source, text);
        assert_eq!(task_name, "Code Review".to_string());
    }
}
