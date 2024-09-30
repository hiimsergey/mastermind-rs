use cursive::theme::{BaseColor, Color};
use cursive::view::Nameable;
use cursive::views::{DummyView, LinearLayout, SliderView, TextView};

/// Signals the reason why the guess provided by the player is invalid
pub enum CodeError {
    /// The guess contains digits outside of the specified `digit_number` setting
    DigitLimit,
    /// The guess contains characters other than letters
    NaN,
    /// The guess is shorter than the specified `code_length` setting
    Short
}

pub fn text_banner() -> TextView {
    TextView::new(
"                     _                      _           _ 
 _ __ ___   __ _ ___| |_ ___ _ __ _ __ ___ (_)_ __   __| |
| '_ ` _ \\ / _` / __| __/ _ \\ '__| '_ ` _ \\| | '_ \\ / _` |
| | | | | | (_| \\__ \\ ||  __/ |  | | | | | | | | | | (_| |
|_| |_| |_|\\__,_|___/\\__\\___|_|  |_| |_| |_|_|_| |_|\\__,_|

"
    )
}

pub fn text_about() -> TextView {
    TextView::new(
"A little game written for the sake of experience in writing
Rust code. Also my first project using a User Interface
library. Rewritten from scratch at v1.0.0.

  <https://github.com/hiimsergey/mastermind-rs>

Utilizes the \"cursive\" crate for building TUIs.

  <https://crates.io/crates/cursive>

v1.0.0  GPL-3.0 Licence"
    ).style(Color::Dark(BaseColor::Blue))
}

pub fn text_rules_1() -> TextView {
    TextView::new(
"
Use the arrow keys or the mouse to navigate.
Press q to close windows and Esc to quit the game.

"
    )
}

pub fn text_rules_2() -> TextView {
    TextView::new(
"A random code is generated based on your settings:

"
    ).style(Color::Dark(BaseColor::Blue))
}

pub fn text_rules_3() -> TextView {
    TextView::new(
"1. \"Digit number\" sets the amount of different
    characters to feature.
2. \"Code length\" sets the length of the generated
    code.

"
    ).style(Color::Dark(BaseColor::Magenta))
}

pub fn text_rules_4() -> TextView {
    TextView::new(
"You try to guess it by filling in the input box.
The game gives you feedback:

"
    ).style(Color::Dark(BaseColor::Blue))
}

pub fn text_rules_5() -> TextView {
    TextView::new(
"1. An exclamation mark means that one character in
    your guess is right.
2. A question mark means that one character is
    featured in the code but on another position.
3. A dot means that a character isn't featured at
    all."
    ).style(Color::Dark(BaseColor::Magenta))
}

pub fn text_settings_ingame(digit_number: u8, code_length: u8) -> TextView {
    TextView::new(
        format!(
"Digit number:    {digit_number}
Code length:     {code_length}"
        )
    ).style(Color::Dark(BaseColor::Blue))
}

pub fn text_quit() -> TextView {
    TextView::new("Do you want to\nquit the game?")
        .style(Color::Dark(BaseColor::Red))
}

/// Returns a slider belonging to the specified `setting` with the text on the side
/// with the specified `description`
pub fn slider_setting(setting: &'static str, description: &str) -> LinearLayout {
    let slider = SliderView::horizontal(8).value(2).on_change(|s, n| {
        s.call_on_name(setting, |v: &mut TextView| {
            v.set_content(format!("{}", n + 2));
        });
    });

    LinearLayout::horizontal()
        .child(TextView::new(description))
        .child(slider).child(DummyView)
        .child(TextView::new("4").with_name(setting))
}
