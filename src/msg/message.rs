use serde::{Deserialize, Serialize};
use chrono::prelude::*;

fn get_task_name(date: &DateTime<Utc>) -> String {
    let s = "Code review ".to_string();
    let date = date.format("%Y-%m-%d").to_string();
    s + &date
}
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
        match self.source {
            Some(ref s) => {
                if let Some(value) = self.get_task_name_from_source_and_text(s) {
                    let mut name = self.user.clone();
                    name.push_str(&value);
                    return name;
                }
                "undefined".to_string()
            }
            None => "undefined".to_string(),
        }
    }

    fn get_task_name_from_source_and_text(&self, s: &String) -> Option<String> {
        if s == "github" {
            return Some(get_task_name(&Utc::now()));
        }
        if s == "rally" {
            if self.text.is_some() {
                let text = self.text.as_deref().unwrap().to_lowercase();
                if text.contains("root cause") {
                    return Some("Root cause analysis".to_string());
                }
                if text.contains("Update") {
                    return Some("Investigated the issue".to_string());
                }
            }
            return Some("Review and support".to_string());
        }
        None
    }
    pub fn get_source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}
