use std::fs::read_to_string;

use callback::save_todo;
use cursive::views::SelectView;
use home;
use model::Todo;
use ui::set_ui;

mod callback;
mod model;
mod ui;

fn main() {
    let mut siv = cursive::default();

    set_ui(&mut siv);
    //finding home dir
    let mut home_dir = match home::home_dir() {
        Some(p) => p,
        _ => {
            panic!("Can't get $HOME !")
        }
    };
    home_dir.push(".ruto");
    //reading file
    match read_to_string(home_dir) {
        Ok(file) => {
            let mut todos: Vec<Todo> = Vec::new();
            for line in file.lines() {
                todos.push(Todo::new(line.to_string()));
            }
            siv.call_on_name("select", |view: &mut SelectView<String>| {
                //writing todos to the SelectView
                for todo in todos.into_iter() {
                    view.add_item_str(todo.to_string())
                }
            });
        }
        _ => {}
    }
    siv.set_global_callback('q', |s| {
        save_todo(s);
        s.quit();
    });
    siv.run();
}
