use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            completed: false,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        if !self.completed {
            self.completed = true;
            self.completed_at = Some(Utc::now());
        }
    }

    pub fn uncomplete(&mut self) {
        if self.completed {
            self.completed = false;
            self.completed_at = None;
        }
    }
}
