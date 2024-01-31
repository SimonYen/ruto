use cursive::traits::*;
use cursive::views::*;
use cursive::Cursive;
use cursive::CursiveRunnable;

use crate::callback::*;

pub fn set_ui(s: &mut CursiveRunnable) {
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select")
        .full_width();
    let buttons = LinearLayout::vertical()
        .child(Button::new("ADD", add_todo))
        .child(Button::new("DELETE", delete_todo))
        .child(DummyView)
        .child(Button::new("QUIT", Cursive::quit));

    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("RUTO")
        .min_size((30, 50))
        .full_screen(),
    );
}
