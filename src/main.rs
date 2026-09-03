mod db;
mod io_utils;
mod jira_database;
mod models;
mod navigator;
mod ui;

use std::rc::Rc;

use io_utils::wait_for_key_press;

use crate::{
    db::JSONFileDatabase, io_utils::get_user_input, jira_database::JiraDatabase,
    navigator::Navigator,
};

fn main() {
    let jira_database = JiraDatabase {
        database: Box::new(JSONFileDatabase {
            file_path: "data/db.json".to_owned(),
        }),
    };

    let mut navigator = Navigator::new(Rc::new(jira_database));

    loop {
        clearscreen::clear().unwrap();

        if let Some(page) = navigator.get_current_page() {
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\n Press any key to continue...",
                    error
                );
                wait_for_key_press();
            };

            let user_input = get_user_input();

            match page.handle_input(user_input.trim()) {
                Err(error) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }
                Ok(action) => {
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!(
                                "Error handling processing user input: {}\nPress any key to continue...",
                                error
                            );
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
