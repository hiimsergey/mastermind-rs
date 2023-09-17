mod logic;
mod util;
use cursive::{
    Cursive,
    event::Key,
    theme::{ BaseColor, Color, },
    view::{ Nameable, Resizable },
    views::{
        Button,
        Dialog,
        DummyView,
        EditView,
        LinearLayout,
        ListView,
        OnEventView,
        Panel,
        ScrollView,
        TextView,
    }
};
use cursive_aligned_view::Alignable;

pub fn menu(s: &mut Cursive) {
    s.add_global_callback(Key::Esc, |s| quit_window(s));
    s.add_global_callback('q', |s| { s.pop_layer(); });
    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(
                        TextView::new(
                            "Game settings"
                        ).style(Color::Dark(BaseColor::Blue))
                    )
                    .child(DummyView)
                    .child(util::setting_digit_num("Digit number:     "))
                    .child(util::setting_pass_len("Password length:  "))
                    .child(DummyView)
                    .child(Button::new("Start game", game))
            ).title("mastermind-rs")
            .button("Rules", rules)
            .button("About", about)
            .button("Quit", |s| s.quit())
        ).on_event('q', |s| s.quit())
        .on_event(Key::Esc, |s| s.quit())
    );
}

fn about(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(util::banner())
                .child(
                    TextView::new(
                        "
A little game written for the sake of experience in writing
Rust code. Also my first project using any kind of User Interface.
                        "
                    ).style(Color::Dark(BaseColor::Blue))
                )
                .child(util::source_button("https://github.com/hiimsergey/mastermind-rs"))
                .child(
                    TextView::new(
                        "
Utilizes the \"cursive\" crate for building TUIs.
                        "
                    ).style(Color::Dark(BaseColor::Blue))
                )
                .child(util::source_button("https://crates.io/crates/cursive"))
                .child(
                    TextView::new("
v. 0.2.0 - GPLv3 License "
                    ).style(Color::Dark(BaseColor::Blue))
                )
        ).button("Ok", |s| { s.pop_layer(); })
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

    let settings = TextView::new(
        format!("
Digit number:    {digit_num}
Password length: {pass_len}"
        )
    ).style(Color::Dark(BaseColor::Blue)).align_center();
    let input = LinearLayout::horizontal()
        .child(TextView::new("Your guess: "))
        .child(EditView::new()
            .on_submit(move |s, name: &str| {
                logic::check_guess(s, name, digit_num, &password);
            })
            .max_content_width(pass_len as usize)
            .with_name("input")
            .fixed_width(pass_len as usize + 1)
        ).align_center();

    let list = Panel::new(
        ScrollView::new(
            ListView::new()
        ).with_name("list")
    ).fixed_height(12).fixed_width(2 * (pass_len as usize) + 14);
    let game_sidebar = LinearLayout::vertical()
        .child(settings)
        .child(DummyView)
        .child(input)
        .child(DummyView.fixed_height(4))
        .child(Button::new("Menu", |s| { s.pop_layer(); }).align_bottom_right())
        .child(Button::new("Ragequit", |s| s.quit() ).align_bottom_right());

    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(list)
                .child(DummyView)
                .child(game_sidebar)
        ).title("Game")
    );
}

fn quit_window(s: &mut Cursive) {
    s.add_layer(
        OnEventView::new(
            Dialog::around(
                TextView::new(
                    "Do you want to \nquit the game?"
                ).style(Color::Dark(BaseColor::Red))
            ).title("Confirm quit")
            .button("No", |s| { s.pop_layer(); })
            .button("Yes", |s| s.quit())
        ).on_event(Key::Esc, |s| { s.pop_layer(); })
        .on_event('q', |s| { s.pop_layer(); })
    );
}

fn rules(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new(
"Use the arrow keys or the mouse to navigate.
Press q to close windows and Esc to quit the game."
                ).align_center())
                .child(util::rules())
        ).title("Rules")
        .button("Ok", |s| { s.pop_layer(); })
    );
}
