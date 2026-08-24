## Introduction

`jira_cli` is a command-line application that implements the core functionality of Jira. It was developed as part of a programming assignment to demonstrate the design and implementation of a production-quality CLI application in Rust.

The project recreates essential Jira workflows through a terminal-first interface, emphasizing clean architecture, idiomatic Rust, and maintainable code.

### Features

* **Epic CRUD** – Create, read, update, and delete epics.
* **Story CRUD** – Create, read, update, and delete stories.

---
## Database Modelling
In this project we persist Epic and Story records in a JSON file to keep things simple.

The JSON model contains the following components:

- `last_item_id` - A global integer ID counter for both Epics and Stories. Each newly created Epic/Story will increment the counter.

- `epics` - A mapping between Epic IDs and actual Epics. An Epic will consist of a list of Stories in the form of Story IDs.

- `stories` - A mapping between Story IDs and the actual Stories.

- `epic` and `story` both have `name`, `description` and `status`.

- `status` can be `Open`, `InProgress`, `Resolved` or `Closed`.

Example of how the JSON file:
```
{
  "last_item_id": 3,
  "epics": {
    "1": {
      "name": "Epic - Project 1",
      "description": "This is Project 1 for the Bootcamp",
      "status": "InProgress",
      "stories": [
        2,
        3
      ]
    }
  },
  "stories": {
    "2": {
      "name": "Story - Project 1 Solution",
      "description": "Please provide full implement for Project 1",
      "status": "Closed"
    },
    "3": {
      "name": "Story - Project 1 README",
      "description": "Please create README file for Project 1",
      "status": "InProgress"
    }
  }
}
```

Unit tests are included in `model.rs` to verify that `Epic` and `Story` are constructed correctly via their `new()` methods.

These tests rely on the `PartialEq` derive on `Epic`, `Story`, and `Status`, which lets `assert_eq!` compare full struct values directly instead of checking each field manually.