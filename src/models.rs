use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => {
                write!(f, "OPEN")
            }
            Self::InProgress => {
                write!(f, "IN PROGRESS")
            }
            Self::Resolved => {
                write!(f, "RESOLVED")
            }
            Self::Closed => {
                write!(f, "CLOSED")
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
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
