use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, PartialEq)]
pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<u32>,
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

#[derive(Debug, PartialEq)]
pub struct Story {
    name: String,
    description: String,
    status: Status,
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

pub struct DBState {
    last_item_id: u32,
    epics: HashMap<u32, Epic>,
    stories: HashMap<u32, Story>,
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
