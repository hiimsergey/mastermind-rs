// TODO move to lib.rs
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
        SelectView,
        TextView
    }
};
use webbrowser;

fn about(s: &mut Cursive) {
    let banner = 
    // TODO mention license
"                     _                      _           _ 
 _ __ ___   __ _ ___| |_ ___ _ __ _ __ ___ (_)_ __   __| |
| '_ ` _ \\ / _` / __| __/ _ \\ '__| '_ ` _ \\| | '_ \\ / _` |
| | | | | | (_| \\__ \\ ||  __/ |  | | | | | | | | | | (_| |
|_| |_| |_|\\__,_|___/\\__\\___|_|  |_| |_| |_|_|_| |_|\\__,_|";
    let description = "
A silly game written for the sake of experience in writing Rust code. Also my first project using any kind of User Interface.

";
    let desc_cursive = "
Utilizes the \"cursive\" crate for building TUIs.

";

    let layout = LinearLayout::vertical()
        // TODO check out TextView to probably make this part more efficient
        .child(TextView::new(banner))
        .child(TextView::new(description))
        // TODO find up right arrow char
        .child(Button::new("source ↗", |_| {
            if !webbrowser::open(
                "https://github.com/hiimsergey/mastermind-rs"
            ).is_ok() {
                eprintln!("Failed to open website, sorry :(")
            }
        }))
        .child(TextView::new(desc_cursive))
        .child(Button::new("source ↗", |_| {
            if !webbrowser::open(
                "https://crates.io/crates/cursive"
            ).is_ok() {
                eprintln!("Failed to open website, sorry :[")
            };
        }));

    s.add_layer(Dialog::around(layout)
        .button("Ok", |s| { s.pop_layer(); })
        .title("About mastermind-rs")
        .fixed_width(64)
    );
}

fn menu(s: &mut Cursive) {
    let menu_buttons = LinearLayout::vertical()
        // TODO map buttons
        .child(Button::new("Start game", game))
        .child(Button::new("Options", |s| s.quit()))
        .child(Button::new("About", |s| about(s)))
        .child(DummyView)
        .child(Button::new("Quit", |s| s.quit()));
    let menu_message = TextView::new(StyledString::styled(
        "Another one of those\nuseless games...",
        Color::Dark(BaseColor::Blue)
    ));

    // TODO add keyboard callbacks
    // everywhere
    s.pop_layer();
    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(menu_message)
        .child(DummyView)
        .child(menu_buttons))
        .title("mastermind-rs")
    );
}

fn game(s: &mut Cursive) {
    let game_buttons = LinearLayout::vertical()
        // TODO map buttons
        .child(TextView::new("TODO show ingame stats"))
        .child(DummyView)
        .child(Button::new("Restart game", menu))
        .child(Button::new("Back to menu", menu))
        .child(DummyView)
        .child(Button::new("Ragequit", |s| s.quit()));

    s.pop_layer();
    s.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(TextView::new("TODO example description"))
        .child(DummyView)
        .child(game_buttons))
        .title("game")
    );
}

fn main() {
    let mut siv = cursive::default();

    menu(&mut siv);

    siv.run();
    // TODO change theme
}
