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
    pub task_name: Option<String>
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
}
