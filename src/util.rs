use cursive::{
    view::Nameable,
    views::{
        Button,
        Dialog,
        DummyView,
        LinearLayout,
        SliderView,
        TextView
    }
};
use crate::menu;
use webbrowser;

pub fn source_button(url: &'static str) -> Button {
    Button::new("source ↗", |s| {
        if !webbrowser::open(url).is_ok() {
            s.clear_global_callbacks('q');
            s.add_global_callback('q', |s| {
                s.pop_layer();
                s.add_global_callback('q', menu);
            });
            s.add_layer(Dialog::info(
"Couldn't connect to the website. :[
Check that you have a web browser installed."
            ).title("Connection error")
            );
        }
    })
}

pub fn setting_char_num(desc: &str) -> LinearLayout {
    let slider = SliderView::horizontal(8)
        .value(2)
        .on_change(|s, n| {
            s.call_on_name("char_num", |v: &mut TextView| {
                v.set_content(format!("{}", n + 2))
            });
        });

    LinearLayout::horizontal()
        .child(TextView::new(desc))
        .child(slider)
        .child(DummyView)
        .child(TextView::new("4").with_name("char_num"))
}

pub fn setting_pass_len(desc: &str) -> LinearLayout {
    let slider = SliderView::horizontal(8)
        .value(2)
        .on_change(|s, n| {
            s.call_on_name("pass_len", |v: &mut TextView| {
                v.set_content(format!("{}", n + 2))
            });
        });

    LinearLayout::horizontal()
        .child(TextView::new(desc))
        .child(slider)
        .child(DummyView)
        .child(TextView::new("4").with_name("pass_len"))
}