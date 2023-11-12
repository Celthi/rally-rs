use super::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HierarchicalRequirement {
    #[serde(flatten)]
    pub artifact: Artifact,
    pub Tasks: Option<EmbeddedObject>,
}
