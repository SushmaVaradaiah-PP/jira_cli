use crate::{
    db::Database,
    models::{DBState, Epic, Status, Story},
};
use anyhow::{Context, Result, anyhow};

pub struct JiraDatabase {
    pub database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(database: Box<dyn Database>) -> Self {
        Self { database }
    }

    // CRUD for Epic
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db = self.database.read_db()?;

        db.last_item_id += 1;
        let new_id = db.last_item_id;

        db.epics.insert(new_id, epic);
        self.database.write_db(&db)?;

        Ok(new_id)
    }

    pub fn read_epic(&self, epic_id: u32) -> Result<Epic> {
        let db = self.database.read_db()?;

        let epic = db
            .epics
            .get(&epic_id)
            .cloned()
            .ok_or_else(|| anyhow!("Epic with ID {} not found", epic_id))?;

        Ok(epic)
    }

    pub fn update_epic_status(&self, epic_id: u32, new_status: Status) -> Result<()> {
        let mut db = self.database.read_db()?;

        let epic = db
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Epic with ID {} not found", epic_id))?;

        epic.status = new_status;

        self.database.write_db(&db)?;

        Ok(())
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut db = self.database.read_db()?;

        let epic = db
            .epics
            .remove(&epic_id)
            .with_context(|| format!("Epic with ID {} not found", epic_id))?;

        for story_id in &epic.stories {
            db.stories.remove(story_id);
        }

        self.database.write_db(&db)?;
        Ok(())
    }

    // CRUD for Story
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db = self.database.read_db()?;

        let epic = db
            .epics
            .get_mut(&epic_id)
            .with_context(|| format!("Epic with ID {} not found", epic_id))?;

        db.last_item_id += 1;
        let new_id = db.last_item_id;

        db.stories.insert(new_id, story);
        epic.stories.push(new_id);

        self.database.write_db(&db)?;

        Ok(new_id)
    }

    pub fn read_story(&self, story_id: u32) -> Result<Story> {
        let db = self.database.read_db()?;

        let story = db
            .stories
            .get(&story_id)
            .cloned()
            .ok_or_else(|| anyhow!("Story with ID {} not found", story_id))?;

        Ok(story)
    }

    pub fn update_story_status(&self, story_id: u32, story_status: Status) -> Result<()> {
        let mut db = self.database.read_db()?;

        let story = db
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow!("Story with ID {} not found", story_id))?;

        story.status = story_status;

        self.database.write_db(&db)?;
        Ok(())
    }

    pub fn delete_story(&self, story_id: u32, epic_id: u32) -> Result<()> {
        let mut db = self.database.read_db()?;

        let epic = db
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Epic with ID {} not found", epic_id))?;

        let story_index = epic
            .stories
            .iter()
            .position(|id| id == &story_id)
            .ok_or_else(|| {
                anyhow!(
                    "Story ID {} is not found in Epic {}'s stories vector",
                    story_id,
                    epic_id
                )
            })?;
        epic.stories.remove(story_index);

        db.stories.remove(&story_id);

        self.database.write_db(&db)?;
        Ok(())
    }
}

pub mod test_utils {
    use std::{cell::RefCell, collections::HashMap};

    use super::*;

    // Type that implements database trait
    pub struct MockDB {
        last_written_state: RefCell<DBState>,
    }

    impl MockDB {
        pub fn new() -> Self {
            MockDB {
                last_written_state: RefCell::new(DBState {
                    last_item_id: 0,
                    epics: HashMap::new(),
                    stories: HashMap::new(),
                }),
            }
        }
    }

    impl Database for MockDB {
        fn read_db(&self) -> Result<DBState> {
            let state = self.last_written_state.borrow().clone();
            Ok(state)
        }

        fn write_db(&self, db_state: &DBState) -> Result<()> {
            let latest_state = &self.last_written_state;
            *latest_state.borrow_mut() = db_state.clone();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils::MockDB;
    use super::*;

    #[test]
    fn create_epic_should_work() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic.clone());

        assert!(result.is_ok());

        let id = result.unwrap();
        let db_state = mock_db.database.read_db().unwrap();

        let expected_id = 1;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert_eq!(db_state.epics.get(&id), Some(&epic));
    }

    #[test]
    fn create_story_should_error_if_invalid_epic_id() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let story = Story::new("".to_owned(), "".to_owned());

        let non_existent_epic_id = 999;

        let result = mock_db.create_story(story, non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn create_story_should_work() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = mock_db.create_story(story.clone(), epic_id);
        assert!(result.is_ok());

        let id = result.unwrap();
        let db_state = mock_db.database.read_db().unwrap();

        let expected_id = 2;

        assert_eq!(id, expected_id);
        assert_eq!(db_state.last_item_id, expected_id);
        assert!(db_state.epics.get(&epic_id).unwrap().stories.contains(&id),);
        assert_eq!(db_state.stories.get(&id), Some(&story));
    }

    #[test]
    fn delete_epic_should_error_if_invalid_epic_id() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = mock_db.delete_epic(non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_epic_should_work() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = mock_db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();

        let result = mock_db.delete_epic(epic_id);
        assert!(result.is_ok());

        let db_state = mock_db.database.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(db_state.epics.get(&epic_id), None);
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn delete_story_should_error_if_invalid_epic_id() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = mock_db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();

        let non_existent_epic_id = 999;

        let result = mock_db.delete_story(story_id, non_existent_epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_story_should_error_if_story_not_found_in_epic() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = mock_db.create_story(story, epic_id);
        assert!(result.is_ok());

        let non_existent_story_id = 999;

        let result = mock_db.delete_story(non_existent_story_id, epic_id);
        assert!(result.is_err());
    }

    #[test]
    fn delete_story_should_work() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);
        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = mock_db.create_story(story, epic_id);
        assert!(result.is_ok());

        let story_id = result.unwrap();

        let result = mock_db.delete_story(story_id, epic_id);
        assert!(result.is_ok());

        let db_state = mock_db.database.read_db().unwrap();

        let expected_last_id = 2;

        assert_eq!(db_state.last_item_id, expected_last_id);
        assert_eq!(
            db_state
                .epics
                .get(&epic_id)
                .unwrap()
                .stories
                .contains(&story_id),
            false
        );
        assert_eq!(db_state.stories.get(&story_id), None);
    }

    #[test]
    fn update_epic_status_should_error_if_invalid_epic_id() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_epic_id = 999;

        let result = mock_db.update_epic_status(non_existent_epic_id, Status::Closed);
        assert!(result.is_err());
    }

    #[test]
    fn update_epic_status_should_work() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);

        assert!(result.is_ok());

        let epic_id = result.unwrap();

        let result = mock_db.update_epic_status(epic_id, Status::Closed);

        assert!(result.is_ok());

        let db_state = mock_db.database.read_db().unwrap();

        assert_eq!(db_state.epics.get(&epic_id).unwrap().status, Status::Closed);
    }

    #[test]
    fn update_story_status_should_error_if_invalid_story_id() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };

        let non_existent_story_id = 999;

        let result = mock_db.update_story_status(non_existent_story_id, Status::Closed);
        assert!(result.is_err());
    }

    #[test]
    fn update_story_status_should_work() {
        let mock_db = JiraDatabase {
            database: Box::new(MockDB::new()),
        };
        let epic = Epic::new("".to_owned(), "".to_owned());
        let story = Story::new("".to_owned(), "".to_owned());

        let result = mock_db.create_epic(epic);

        let epic_id = result.unwrap();

        let result = mock_db.create_story(story, epic_id);

        let story_id = result.unwrap();

        let result = mock_db.update_story_status(story_id, Status::Closed);

        assert!(result.is_ok());

        let db_state = mock_db.database.read_db().unwrap();

        assert_eq!(
            db_state.stories.get(&story_id).unwrap().status,
            Status::Closed
        );
    }
}
