use std::fs;

use anyhow::{Context, Result};

use crate::models::{DBState, Epic, Status, Story};

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let file_content = fs::read_to_string(&self.file_path)
            .with_context(|| format!("Failed to read the file in this path {}", self.file_path))?;

        let deserialized_db_state: DBState = serde_json::from_str(&file_content)
            .with_context(|| "Failed to deserialize database content into DBState.")?;

        Ok(deserialized_db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let serialized_db_state =
            serde_json::to_vec(&db_state).with_context(|| "Failed to serialize DBState to JSON")?;

        fs::write(&self.file_path, serialized_db_state).with_context(|| {
            format!(
                "Failed to write the serialized DBState to file in the path {}",
                self.file_path,
            )
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {

        use super::*;
        use std::{collections::HashMap, io::Write};

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase {
                file_path: "INVALID_PATH".to_owned(),
            };
            let result = db.read_db();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                format!("Failed to read the file in this path {}", db.file_path)
            );
        }

        #[test]
        fn read_file_should_fail_for_invalid_parsed_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{"last_item_id":0 epics:{} stories {}}"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(
                result.unwrap_err().to_string(),
                "Failed to deserialize database content into DBState."
            );
        }

        #[test]
        fn read_file_should_fail_for_valid_json_but_wrong_db_state() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{"last_item_id":0, "epics":{}}"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("Failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert_eq!(
                result.unwrap_err().to_string(),
                "Failed to deserialize database content into DBState."
            );
        }

        #[test]
        fn read_file_should_parse_and_deserialize_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_content = r#"{"last_item_id": 0, "epics":{},"stories":{}}"#;
            write!(tmpfile, "{}", file_content).unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let result = db.read_db();

            assert!(result.is_ok());
        }

        #[test]
        fn write_db_should_fail_when_file_passed_does_not_exist() {
            let db = JSONFileDatabase {
                file_path: "nonexistent_dir/test_db.json".to_string(),
            };

            let db_state = DBState {
                last_item_id: 0,
                epics: HashMap::new(),
                stories: HashMap::new(),
            };

            let result = db.write_db(&db_state);
            assert_eq!(
                result.unwrap_err().to_string(),
                format!(
                    "Failed to write the serialized DBState to file in the path {}",
                    db.file_path,
                )
            );
        }

        #[test]
        fn write_db_should_serialize_and_write_to_file() {
            let tmpfile = tempfile::NamedTempFile::new().unwrap();

            let db = JSONFileDatabase {
                file_path: tmpfile
                    .path()
                    .to_str()
                    .expect("failed to convert tmpfile path to str")
                    .to_string(),
            };

            let story = Story {
                name: "Story 1".to_owned(),
                description: "Story 1 Description".to_owned(),
                status: Status::Open,
            };

            let epic = Epic {
                name: "Epic 1".to_owned(),
                description: "Epic 1 description".to_owned(),
                status: Status::Open,
                stories: vec![2],
            };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState {
                last_item_id: 2,
                epics,
                stories,
            };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            assert_eq!(read_result, state);
        }
    }
}
