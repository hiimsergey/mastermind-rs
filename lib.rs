mod util;
use cursive::{
    Cursive,
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
        EditView,
        LinearLayout,
        OnEventView,
        SelectView,
        TextView
    }
};

pub fn menu(s: &mut Cursive) {
    // TODO dont forget to move a generic version to util.rs
    // TODO use this or delete if u become desperate
    // TODO idea: re-call menu but add arguments for this func
    // them being a tripel containing the setting values
    // the last being an Option value
    // TODO yeah just impl another view
    let mut label = 5;
    let amogus = LinearLayout::horizontal()
        .child(TextView::new("TODO amogus:      "))
        .child(OnEventView::new(
            Button::new(format!("<{label}>"), menu))
            .on_event('x', move |s| {
                label += 1;
                s.quit();
            })
        );

    // TODO test of time
    // and every other /global_callback/ too
    s.add_global_callback('q', |s| s.quit());
    s.pop_layer();
    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(TextView::new(StyledString::styled(
            "Start new game:",
            Color::Dark(BaseColor::Blue)
        )))
        .child(DummyView)
        // TODO submitting makes the cursor move to the next editview
        // TODO implement left/right keys using OnEventView
        // TODO next write this ^
        // TODO amogus
        .child(amogus)
        .child(TextView::new("Character number"))
        .child(TextView::new("Password length"))
        .child(TextView::new("Guess limit"))
        ).title("mastermind-rs")
        // TODO also bind Esc for quit
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
    let description = "
A little game written for the sake of experience in writing Rust code. Also my first project using any kind of User Interface.

";
    let desc_cursive = "
Utilizes the \"cursive\" crate for building TUIs.

";

    let layout = LinearLayout::vertical()
        // TODO check out TextView to probably make this part more efficient
        .child(TextView::new(banner))
        .child(TextView::new(StyledString::styled(
            description,
            Color::Dark(BaseColor::Blue)
        )))
        .child(util::source_button("https://github.com/hiimsergey/mastermind-rs"))
        .child(TextView::new(StyledString::styled(
            desc_cursive,
            Color::Dark(BaseColor::Blue)
        )))
        .child(util::source_button("https://crates.io/crates/cursive"));

    // TODO dont forget to impl this for other menu-originating functions
    s.clear_global_callbacks('q');
    s.add_global_callback('q', |s| { s.pop_layer(); menu(s) });
    s.pop_layer();
    s.add_layer(Dialog::around(layout)
        .button("Ok", |s| { s.pop_layer(); menu(s) })
        .title("About mastermind-rs")
        .fixed_width(64)
    );
}

fn rules(s: &mut Cursive) {
    let rules = StyledString::styled(
"
A random password in generated based on your settings:

1. \"Character number\" sets the amount of different
   characters to feature.
2. \"Password length\" is self-explanatory, I guess.
3. \"Guess limit\" spawns a Game Over screen once this
   number of guesses is exceeded.

You try to guess the password by filling in the text box.
The game gives you feedback:

1. A red star (*) means that one character in your guess
   is right.
2. A white star (*) means that one character is featured
   in the password but on another position.",
        Color::Dark(BaseColor::Blue)
    );

    s.clear_global_callbacks('q');
    s.add_global_callback('q', |s| { s.pop_layer(); menu(s) });
    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(TextView::new("(Use the arrow keys or the mouse to navigate)"))
        .child(TextView::new(rules))
    ).title("Rules")
    .button("Ok", |s| { s.pop_layer(); menu(s) }));
}
