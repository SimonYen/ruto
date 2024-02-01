use std::fs::File;
use std::io::Write;

use cursive::views::*;
use cursive::Cursive;

use cursive::traits::*;

use chrono::prelude::*;

//add a new todo
pub fn add_todo(s: &mut Cursive) {
    fn ok(s: &mut Cursive, todo: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            //get current time
            let local = Local::now().format("%H:%M:%S").to_string();
            view.add_item_str(format!("{}\t{}", todo, local))
        });
        s.pop_layer();
    }
    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("edit")
                .fixed_size((50, 20)),
        )
        .title("Input a new todo")
        .button("save", |s| {
            let todo = s
                .call_on_name("edit", |view: &mut EditView| view.get_content())
                .expect("Can't get EditView content !");
            ok(s, &todo);
        })
        .button("cancel", |s| {
            s.pop_layer();
        }),
    );
}

//delete selected todo
pub fn delete_todo(s: &mut Cursive) {
    let mut select = s
        .find_name::<SelectView<String>>("select")
        .expect("can't find SelectView !");
    match select.selected_id() {
        Some(target) => {
            select.remove_item(target);
        }
        _ => s.add_layer(Dialog::info("None !")),
    }
}

/*callback function called when todo has been pressed*/
pub fn on_press(s: &mut Cursive, todo: &str) {
    s.add_layer(
        Dialog::text(todo)
            .title("DETAIL")
            .button("ok", |s| {
                s.pop_layer();
            })
            .fixed_size((30, 10)),
    );
}

//save all todo before quit
pub fn save_todo(s: &mut Cursive) {
    //get all todos
    s.call_on_name("select", |view: &mut SelectView<String>| {
        let mut todos: Vec<String> = Vec::new();
        for (todo, _) in view.iter() {
            let mut t = todo.to_string();
            t.push('\n');
            todos.push(t);
        }
        let mut home_dir = home::home_dir().unwrap();
        home_dir.push(".ruto");
        let mut f = File::create(home_dir).unwrap();
        for todo in todos.into_iter() {
            let _ = f.write(todo.as_bytes());
        }
    });
}
