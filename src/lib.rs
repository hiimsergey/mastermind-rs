mod util;
use cursive::{
    Cursive,
    event,
    theme::{
        BaseColor,
        Color,
    },
    utils::markup::StyledString,
    view::Resizable,
    views::{
        Button,
        Dialog,
        DummyView,
        LinearLayout,
        TextView
    }
};

// TODO eventually frame the settings to visually isolate from rules-about-quit buttons
pub fn menu(s: &mut Cursive) {
    // TODO find T02
    s.add_global_callback(event::Key::Esc, |s| s.quit());
    // TODO if the last window gets closed, the program stops
    // if game closes, menu pops up
    s.add_global_callback('q', |s| { s.pop_layer(); });
    s.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new(StyledString::styled(
                "Game settings:",
                Color::Dark(BaseColor::Blue)
            )))
            .child(DummyView)
            .child(util::setting_char_num("Character number: "))
            .child(util::setting_pass_len("Password length:  "))
            .child(DummyView)
            .child(Button::new("Start game", game))
        ).title("mastermind-rs")
        .button("Rules", rules)
        .button("About", about)
        .button("Quit", |s| s.quit())
    );
}

fn about(s: &mut Cursive) {
    // TODO mention license
    let banner =
"                     _                      _           _ 
 _ __ ___   __ _ ___| |_ ___ _ __ _ __ ___ (_)_ __   __| |
| '_ ` _ \\ / _` / __| __/ _ \\ '__| '_ ` _ \\| | '_ \\ / _` |
| | | | | | (_| \\__ \\ ||  __/ |  | | | | | | | | | | (_| |
|_| |_| |_|\\__,_|___/\\__\\___|_|  |_| |_| |_|_|_| |_|\\__,_|";

    let layout = LinearLayout::vertical()
        // TODO check out TextView to probably make this part more efficient
        .child(TextView::new(banner))
        .child(TextView::new(StyledString::styled(
            "
A little game written for the sake of experience in writing Rust code. Also my first project using any kind of User Interface.
            ",
            Color::Dark(BaseColor::Blue)
        )))
        .child(util::source_button("https://github.com/hiimsergey/mastermind-rs"))
        .child(TextView::new(StyledString::styled(
            "
Utilizes the \"cursive\" crate for building TUIs.
            ",
            Color::Dark(BaseColor::Blue)
        )))
        .child(util::source_button("https://crates.io/crates/cursive"));

    // TODO dont forget to impl this for other menu-originating functions
    s.add_layer(Dialog::around(layout)
        .button("Ok", |s| { s.pop_layer(); })
        .title("About mastermind-rs")
        .fixed_width(64)
    );
}

fn game(s: &mut Cursive) {
    // TODO add global callback

    let char_num = s.call_on_name("char_num", |v: &mut TextView| {
        let binding = v.get_content();
        binding.source().parse::<u8>().unwrap()
    }).unwrap();
    let pass_len = s.call_on_name("pass_len", |v: &mut TextView| {
        let binding = v.get_content();
        binding.source().parse::<u8>().unwrap()
    }).unwrap();
    let buttons = LinearLayout::vertical()
        .child(Button::new("Back to menu", |s| { s.pop_layer(); menu(s) }))
        .child(DummyView)
        .child(Button::new("Ragequit", |s| s.quit()));


    s.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(TextView::new("TODO new"))
        // TODO evtl resize
        .child(DummyView)
        .child(buttons)
    ).title("Game"));
}

fn rules(s: &mut Cursive) {
    let rules = StyledString::styled(
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
    );

    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(TextView::new("(Use the arrow keys or the mouse to navigate)"))
        .child(TextView::new(rules))
    ).title("Rules")
    .button("Ok", |s| { s.pop_layer(); }));
}