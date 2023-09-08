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
A simple game written for the sake of experience in writing Rust code. Also my first project using any kind of User Interface.

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
        .child(Button::new("source ↗", |s| {
            if !webbrowser::open(
                "https://github.com/hiimsergey/mastermind-rs"
            ).is_ok() {
                source_error(s);
            }
        }))
        .child(TextView::new(StyledString::styled(
            desc_cursive,
            Color::Dark(BaseColor::Blue)
        )))
        .child(Button::new("source ↗", |s| {
            if !webbrowser::open(
                "https://crates.io/crates/cursive"
            ).is_ok() {
                source_error(s);
            };
        }));

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

fn menu(s: &mut Cursive) {
    // TODO test of time
    // and every other /global_callback/ too
    s.add_global_callback('q', |s| s.quit());
    s.pop_layer();
    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(TextView::new(StyledString::styled(
            "Another one of those useless games...",
            Color::Dark(BaseColor::Blue)
        )))
        .child(DummyView)
        // TODO now -> EditView
        .child(TextView::new("Character number"))
        .child(TextView::new("Password length"))
        .child(TextView::new("HP/Round limit")))
        // TODO make all DummyViews larger
        .title("mastermind-rs")
        .button("About", about)
        .button("Quit", |s| s.quit())
    );
}

fn source_error(s: &mut Cursive) {
    s.clear_global_callbacks('q');
    s.add_global_callback('q', |s| {
        s.pop_layer();
        s.add_global_callback('q', menu);
    });
    s.add_layer(Dialog::text(
"Couldn't connect to the website. :[
Check that you have a web browser installed.")
        .title("Connection error")
        .button("Ok", |s| { s.pop_layer(); })
    );
}

fn main() {
    let mut siv = cursive::default();
    menu(&mut siv);
    siv.run();
    // TODO change theme
}
