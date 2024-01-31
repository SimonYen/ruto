use cursive::views::*;
use cursive::Cursive;

use cursive::traits::*;

//add a new todo
pub fn add_todo(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
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
            let name = s
                .call_on_name("edit", |view: &mut EditView| view.get_content())
                .expect("Can't get EditView content !");
            ok(s, &name);
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
    s.add_layer(Dialog::text(todo).title("DETAIL").button("ok", |s| {
        s.pop_layer();
    }));
}
