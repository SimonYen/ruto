use cursive::traits::*;
use cursive::views::*;
use cursive::CursiveRunnable;

use crate::callback::*;

pub fn set_ui(s: &mut CursiveRunnable) {
    let select = SelectView::<String>::new()
        .h_align(cursive::align::HAlign::Center)
        .on_submit(on_press)
        .with_name("select");
    let buttons = LinearLayout::horizontal()
        .child(Button::new("ADD", add_todo))
        .child(Button::new("DELETE", delete_todo))
        .child(DummyView.full_width())
        .child(Button::new("QUIT", |s| {
            save_todo(s);
            s.quit();
        }));

    //creating two panels
    let list_panel = Panel::new(select).title("list").full_height();
    let btn_panel = Panel::new(buttons).title("operation");

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(list_panel)
                .child(DummyView)
                .child(btn_panel),
        )
        .title_position(cursive::align::HAlign::Left)
        .title("RUTO")
        .full_screen(),
    );
}
