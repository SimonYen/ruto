use cursive::traits::*;
use cursive::views::*;
use cursive::Cursive;
use cursive::CursiveRunnable;

use crate::callback::*;

pub fn set_ui(s: &mut CursiveRunnable) {
    let select = SelectView::<String>::new()
        .on_submit(on_press)
        .with_name("select");
    let buttons = LinearLayout::vertical()
        .child(Button::new("ADD", add_todo))
        .child(Button::new("DELETE", delete_todo))
        .child(DummyView.full_height())
        .child(Button::new("QUIT", Cursive::quit));

    //creating two panels
    let list_panel = Panel::new(select).title("list").full_width();
    let btn_panel = Panel::new(buttons).title("operation");

    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(list_panel)
                .child(DummyView)
                .child(btn_panel),
        )
        .title_position(cursive::align::HAlign::Left)
        .title("RUTO")
        .min_size((30, 50))
        .full_screen(),
    );
}
