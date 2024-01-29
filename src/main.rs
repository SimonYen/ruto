use cursive::views::*;
use cursive::Cursive;

use cursive::traits::*;

fn main() {
    let mut siv = cursive::default();
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select");
    let buttons = LinearLayout::vertical()
        .child(Button::new("添加", add_name))
        .child(Button::new("删除", delete_name))
        .child(DummyView)
        .child(Button::new("退出", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("请选择")
        .full_screen(),
    );

    siv.set_global_callback('q', |s| s.quit());
    siv.run();
}

fn add_name(s: &mut Cursive) {
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
                .with_name("name")
                .fixed_size((50, 20)),
        )
        .title("输入一个新名字")
        .button("确认", |s| {
            let name = s
                .call_on_name("name", |view: &mut EditView| view.get_content())
                .expect("无法获取编辑框内容!");
            ok(s, &name);
        })
        .button("取消", |s| {
            s.pop_layer();
        }),
    );
}

fn delete_name(s: &mut Cursive) {
    let mut select = s
        .find_name::<SelectView<String>>("select")
        .expect("找不到该组件!");
    match select.selected_id() {
        Some(target) => {
            select.remove_item(target);
        }
        _ => s.add_layer(Dialog::info("没有删除的东西")),
    }
}

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("hhh")
            .title(name)
            .button("退出", Cursive::quit),
    );
}
