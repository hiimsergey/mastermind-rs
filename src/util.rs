use cursive::{
    theme::{
        BaseColor,
        Color
    },
    view::Nameable,
    views::{
        Button,
        Dialog,
        DummyView,
        LinearLayout,
        SliderView,
        TextView
    }, utils::markup::StyledString
};
use webbrowser;

// TODO write this                  // "&str": im not sure
// pub fn check_valid_input(password: &str) -> Result<(), FormatError> {}

// TODO then write this
// enum FormatError {
//     Short,
//     NaN,
//     WrongChar
// }

// TODO and somewhere, write this
// s.add_layer(Dialog::info(match valid_guess {
//     FormatError::Short => "The provided guess is too short.",
//     FormatError::Nan => "The guess is not a number.",
//     FormatError::WrongChar => "The guess contains a digit beyond the character number limit."
// }))

pub fn source_button(url: &'static str) -> Button {
    Button::new("source ↗", |s| {
        if !webbrowser::open(url).is_ok() {
            s.add_global_callback('q', |s| { s.pop_layer(); });
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

pub fn banner() -> TextView {
    TextView::new(
"                     _                      _           _ 
 _ __ ___   __ _ ___| |_ ___ _ __ _ __ ___ (_)_ __   __| |
| '_ ` _ \\ / _` / __| __/ _ \\ '__| '_ ` _ \\| | '_ \\ / _` |
| | | | | | (_| \\__ \\ ||  __/ |  | | | | | | | | | | (_| |
|_| |_| |_|\\__,_|___/\\__\\___|_|  |_| |_| |_|_|_| |_|\\__,_|"
    )
}
    
pub fn rules() -> StyledString {
    StyledString::styled(
"
A random password is generated based on your settings:

1. \"Character number\" sets the amount of different
    characters to feature.
2. \"Password length\" sets the length of the generated
    password.

You try to guess it by filling in the input box.
The game gives you feedback:

1. A red star (*) means that one character in your guess
    is right.
2. A white star means that one character is featured in
    the password but on another position.",
        Color::Dark(BaseColor::Blue)
    )
}