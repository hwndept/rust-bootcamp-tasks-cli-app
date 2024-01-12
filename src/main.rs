use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(TasksDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        let page = navigator.get_current_page();
        if let Some(page) = page {
            if let Err(e) = page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", e);

                wait_for_key_press();
            } else {
                let input = get_user_input();
                let input = input.trim();
                let action = page.handle_input(input);
                if let Ok(Some(action)) = action {
                    navigator.handle_action(action).unwrap();
                }
            }
        } else {
            break;
        }
    }
}
