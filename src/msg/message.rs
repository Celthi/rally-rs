use super::task_content::TaskContentMap;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct TimeSpent {
    user: String,
    login: String,
    value: f32,
    id: u64,
    wp_formatted_id: Option<String>,
    repo_name: Option<String>,
    pr_number: Option<u64>,
    pub task_name: Option<String>,
    source: Option<String>,
    text: Option<String>,
}

impl TimeSpent {
    pub fn get_user_name(&self) -> &str {
        &self.user
    }
    pub fn get_login_name(&self) -> &str {
        &self.login
    }
    pub fn get_time_spent(&self) -> f32 {
        self.value
    }
    pub fn get_wp_formatted_id(&self) -> Option<String> {
        self.wp_formatted_id.clone()
    }
    pub fn get_repo_name(&self) -> Option<&str> {
        self.repo_name.as_deref()
    }
    pub fn get_pr_number(&self) -> Option<u64> {
        self.pr_number
    }
    pub fn get_task_name(&self) -> String {
        if self.task_name.is_some() {
            return self.task_name.as_deref().unwrap().to_string();
        }
        if self.source.is_some() {
            if let Some(value) = self.get_task_name_from_source_and_text() {
                let mut name = self.user.clone();
                name.push_str(&value);
                if self.source.as_deref().unwrap().contains("github") {
                    name.push_str(Utc::now().format("%Y-%m-%d").to_string().as_str());
                }
                return name;
            }
        }
        "undefined".to_string()
    }

    fn get_task_name_from_source_and_text(&self) -> Option<String> {
        Some(TaskContentMap::get_task_name(self.source.as_deref()?, self.text.as_deref()?))
    }
    pub fn get_source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}
