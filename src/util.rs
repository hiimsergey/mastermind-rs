// util.rs - some helper functions for the other files

use cursive::{
    theme::{ BaseColor, Color },
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
use webbrowser;

pub enum FormatError {
    DigitLimit,
    NaN,
    Short
}

pub fn source_button(url: &'static str) -> Button {
    Button::new("source ↗", |s| {
        if !webbrowser::open(url).is_ok() {
            s.add_global_callback('q', |s| { s.pop_layer(); });
            s.add_layer(Dialog::info(
"Couldn't connect to the website. :[
Check that you have a web browser installed."
            ).title("Connection error"));
        }
    })
}

pub fn setting_digit_num(desc: &str) -> LinearLayout {
    let slider = SliderView::horizontal(8)
        .value(2)
        .on_change(|s, n| {
            s.call_on_name("digit_num", |v: &mut TextView| {
                v.set_content(format!("{}", n + 2))
            });
        });

    LinearLayout::horizontal()
        .child(TextView::new(desc))
        .child(slider)
        .child(DummyView)
        .child(TextView::new("4").with_name("digit_num"))
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

pub fn banner() -> TextView {
    TextView::new(
"                     _                      _           _ 
 _ __ ___   __ _ ___| |_ ___ _ __ _ __ ___ (_)_ __   __| |
| '_ ` _ \\ / _` / __| __/ _ \\ '__| '_ ` _ \\| | '_ \\ / _` |
| | | | | | (_| \\__ \\ ||  __/ |  | | | | | | | | | | (_| |
|_| |_| |_|\\__,_|___/\\__\\___|_|  |_| |_| |_|_|_| |_|\\__,_|"
    )
}
    
pub fn rules() -> TextView {
    TextView::new(
"
A random code is generated based on your settings:

1. \"Digit number\" sets the amount of different
    characters to feature.
2. \"Code length\" sets the length of the generated
    code.

You try to guess it by filling in the input box.
The game gives you feedback:

1. An exclamation mark means that one character in
   your guess is right.
2. A question mark means that one character is
   featured in the code but on another position.
3. A dot means that a character isn't featured at
   all."
    ).style(Color::Dark(BaseColor::Blue))
}
