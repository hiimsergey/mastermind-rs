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
        // TODO r u sure about that
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
        TextView,
    }
};
use cursive_aligned_view::Alignable;

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

// TODO impl Scrollable trait
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

    // TODO make center or make a box around it
    let guesses = LinearLayout::horizontal()
        .child(Panel::new(ListView::new()
            .with_name("list")
            .fixed_height(10)
            .fixed_width(pass_len as usize + 5)
        ))
        // TODO dummy
        .child(DummyView.fixed_width((pass_len as usize) / 2))
        // TODO evtl add label here too (see ^)
        .child(Panel::new(ListView::new()
            .with_name("feedback")
            .fixed_height(10)
            .fixed_width(pass_len as usize + 1)
        ));
    let input = LinearLayout::horizontal()
        .child(TextView::new("Your guess: "))
        .child(EditView::new()
            // TODO big update incoming
            .on_submit(move |s, name: &str| {
                logic::check_guess(s, name, digit_num, &password);
            })
            .max_content_width(pass_len as usize)
            .fixed_width(pass_len as usize + 1)
        ).align_bottom_right();

    s.add_layer(Dialog::around(LinearLayout::vertical()
            .child(guesses.align_center())
            .child(DummyView)
            .child(input)
        ).title("Game")
        .button("Back to menu", |s| { s.pop_layer(); })
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