use std::rc::Rc;

use db::JiraDatabase;
use io_utils::*;
use navigators::*;

mod db;
mod io_utils;
mod models;
mod navigators;
mod ui;

fn main() {
    println!("Hello, It's Jira CLI");

    // Create a new instance of the JiraDatabase
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    // Create a new instance of the Navigator
    let mut navigator = Navigator::new(Rc::clone(&db));

    // Start the navigator
    loop {
        // Clear the screen before rendering the next page
        clearscreen::clear().unwrap();

        // Get the current page from the navigator
        if let Some(page) = navigator.get_current_page() {
            // Draw the page to the screen
            if let Err(error) = page.draw_page() {
                println!(
                    "Error rendering page: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
            };

            // Get user input from the command line
            let user_input = get_user_input();

            // Handle the user input and get the action to perform
            match page.handle_input(user_input.trim()) {
                // Handle any errors that occur when getting user input
                Err(error) => {
                    println!(
                        "Error getting user input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                }

                Ok(action) => {
                    // Handle the action returned by the page
                    if let Some(action) = action {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling processing user input: {}\nPress any key to continue...", error);
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
