mod logic;
mod util;
use cursive::{
    Cursive,
    event,
    utils::markup::StyledString,
    theme::{
        BaseColor,
        Color,
    },
    view::{
        Nameable,
        Resizable
    },
    views::{
        Button,
        Dialog,
        DummyView,
        EditView,
        LinearLayout,
        ListView,
        Panel,
        ScrollView,
        TextView,
    }
};
use cursive_aligned_view::Alignable;

// TODO or maybe substitute feedback for a color canvas
pub fn menu(s: &mut Cursive) {
    s.add_global_callback(event::Key::Esc, |s| s.quit());
    // TODO if the last window gets closed, the program stops
    s.add_global_callback('q', |s| { s.pop_layer(); });
    s.add_layer(Dialog::around(LinearLayout::vertical()
            .child(TextView::new(StyledString::styled(
                "Game settings:",
                Color::Dark(BaseColor::Blue)
            )))
            .child(DummyView)
            .child(util::setting_digit_num("Digit number:     "))
            .child(util::setting_pass_len("Password length:  "))
            .child(DummyView)
            // TODO big update incoming
            .child(Button::new("Start game", game))
        ).title("mastermind-rs")
        .button("Rules", rules)
        .button("About", about)
        .button("Quit", |s| s.quit())
    );
}

fn about(s: &mut Cursive) {
    // TODO mention license
    let layout = LinearLayout::vertical()
        // TODO check out TextView to probably make this part more efficient
        .child(util::banner())
        .child(TextView::new(StyledString::styled(
            "
A little game written for the sake of experience in writing
Rust code. Also my first project using any kind of User Interface.
            ",
            // TODO use TextView::style to color
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

    s.add_layer(Dialog::around(layout)
        .button("Ok", |s| { s.pop_layer(); })
        .title("About mastermind-rs")
        .fixed_width(64)
    );
}

fn game(s: &mut Cursive) {
    let digit_num: u8 = s.call_on_name("digit_num", |v: &mut TextView| {
        let binding = v.get_content();
        binding.source().parse().unwrap()
    }).unwrap();
    let pass_len: u8 = s.call_on_name("pass_len", |v: &mut TextView| {
        let binding = v.get_content();
        binding.source().parse().unwrap()
    }).unwrap();
    let password = logic::gen_password(digit_num, pass_len);

    // TODO readd Panel
    // TODO restructure all the windows like this
    // TODO think about moving these little Views into views.rs
    let settings = TextView::new(
            StyledString::styled(
            format!("Digit number:    {digit_num}\nPassword length: {pass_len}"),
            Color::Dark(BaseColor::Blue)
        )
    ).align_center();
    let list = Panel::new(
        ScrollView::new(
            ListView::new()
        ).with_name("list")
    ).fixed_height(12).fixed_width(2 * (pass_len as usize) + 14);
    // TODO readd
    // .align_center();
    let input = LinearLayout::horizontal()
        .child(TextView::new("Your guess: "))
        .child(EditView::new()
            .on_submit(move |s, name: &str| {
                s.call_on_name("input", |v: &mut EditView| {
                    v.set_content("");
                });
                logic::check_guess(s, name, digit_num, &password);
            })
            .max_content_width(pass_len as usize)
            .with_name("input")
            .fixed_width(pass_len as usize + 1)
        ).align_center();

    s.add_layer(Dialog::around(LinearLayout::vertical()
            .child(settings)
            .child(list)
            .child(DummyView)
            .child(input)
        ).title("Game")
        .button("Menu", |s| { s.pop_layer(); })
        .button("Ragequit", |s| s.quit())
    );
    // TODO reject input if not according to rules
    // and not a number
    // and too *short*
}

fn rules(s: &mut Cursive) {
    s.add_layer(Dialog::around(LinearLayout::vertical()
        .child(TextView::new(
"Use the arrow keys or the mouse to navigate.
Press q to close windows and Esc to quit the game."
).align_center())
        .child(TextView::new(util::rules()))
    ).title("Rules")
    .button("Ok", |s| { s.pop_layer(); }));
}