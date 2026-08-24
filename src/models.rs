use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

#[cfg(test)]
mod tests {
    use super::{Epic, Status, Story};

    #[test]
    fn create_an_epic() {
        let epic = Epic::new(
            "First Epic".to_owned(),
            "Description for first epic".to_string(),
        );

        assert_eq!(
            epic,
            Epic {
                name: "First Epic".to_owned(),
                description: "Description for first epic".to_owned(),
                status: Status::Open,
                stories: vec![]
            }
        );
    }

    #[test]
    fn create_a_story() {
        let story = Story::new(
            "First Story".to_owned(),
            "Description for first story".to_owned(),
        );

        assert_eq!(
            story,
            Story {
                name: "First Story".to_owned(),
                description: "Description for first story".to_owned(),
                status: Status::Open
            }
        );
    }
}
