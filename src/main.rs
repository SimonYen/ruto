use ui::set_ui;

mod callback;
mod ui;

fn main() {
    let mut siv = cursive::default();

    set_ui(&mut siv);

    siv.set_global_callback('q', |s| s.quit());
    siv.run();
}
