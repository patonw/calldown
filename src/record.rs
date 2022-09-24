use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub date: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub pinned: Option<bool>,
    pub general: String,
    pub ours: String,
    pub exclude: String,
}

impl Record {
    pub fn with_ours(&self, value: &str) -> Self {
        let mut result = self.clone();
        result.ours = value.to_string();
        result
    }
}


