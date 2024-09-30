use cursive::Cursive;
use cursive::event::Key;
use cursive::theme::{BaseColor, Color};
use cursive::view::{Nameable, Resizable, ScrollStrategy};
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout,
    ListView, OnEventView, Panel, ScrollView, TextView};
use crate::{logic, util, util::CodeError, windows};

/// Spawns the menu windows, i.e. the first one you see when booting the game
pub fn menu(s: &mut Cursive) {
    // Pressing Esc anywhere but the menu will open the confirmation window
    // Pressing q will close the current window
    s.add_global_callback(Key::Esc, confirm_quit);
    s.add_global_callback('q', |s| { s.pop_layer(); });

    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(TextView::new("Game settings")
                        .style(Color::Dark(BaseColor::Blue))
                    ).child(DummyView)
                    .child(util::slider_setting("digit_number", "Digit number:    "))
                    .child(util::slider_setting("code_length", "Code length:     "))
                    .child(DummyView)
                    .child(Button::new("Start game", game))
            )
            .title("mastermind-rs")
            .button("Rules", rules)
            .button("About", about)
            .button("Quit", |s| s.quit())
        )
        // Pressing the keys below will open the respective windows
        .on_event('a', about)
        .on_event('g', game)
        .on_event('s', game)
        .on_event('q', |s| s.quit())
        .on_event('r', rules)
        // Pressing Esc in the menu window will just quit the game
        .on_event(Key::Esc, |s| s.quit())
    );
}

/// Spawns a window informing the player that their guess is invalid
pub fn guess_error(s: &mut Cursive, err: CodeError) {
    s.add_layer(
        Dialog::around(
            TextView::new(match err {
                CodeError::DigitLimit => "The given guess contains\ndigits outside the limit.",
                CodeError::NaN => "The given guess is\nnot a valid number.",
                CodeError::Short => "The given guess\nis too short."
            }).style(Color::Dark(BaseColor::Red))
        )
            .title("Invalid input")
            .button("Ok", |s| { s.pop_layer(); })
    );
}

/// Spawns a window concluding the game and congratulating the player on winning
pub fn win(s: &mut Cursive, code: &str) {
    s.add_layer(
        Dialog::around(
            TextView::new(format!("Congratulations!\nThe code was {code}."))
                .style(Color::Dark(BaseColor::Blue))
        )
            .title("You won!")
            .button("Ok", |s| { s.pop_layer(); })
            .button("Play again", |s| {
                // Pops twice so that it brings you to the menu
                // win -pop-> game -pop-> menu
                s.pop_layer();
                s.pop_layer();
                windows::game(s);
            })
            .button("Menu", |s| {
                s.pop_layer();
                s.pop_layer();
            })
    );
}

/// Spawns a window with information about the game
fn about(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(util::text_banner())
                .child(util::text_about())
        )
            .button("Ok", |s| { s.pop_layer(); })
            .title("About mastermind-rs")
            .fixed_width(64)
    );
}

/// Spawns the ingame window
fn game(s: &mut Cursive) {
    // Reads the settings the player configured with the sliders
    let digit_number: u8 = s.call_on_name("digit_number", |v: &mut TextView| v
            .get_content()
            .source()
            .parse().unwrap()
        ).unwrap();
    let code_length: u8 = s
        .call_on_name("code_length", |v: &mut TextView| v
            .get_content()
            .source()
            .parse().unwrap()
        ).unwrap();
    // Generates the code that the player should find
    let code = logic::get_code(digit_number, code_length);

    // information about the settings the player configured
    let ingame_settings = util::text_settings_ingame(digit_number, code_length);
    // bar where you write your guess
    let input_bar = LinearLayout::horizontal()
        .child(TextView::new("Your guess: "))
        .child(EditView::new()
            .on_submit(move |s, guess: &str| {
                logic::submit_guess(s, guess, &code, digit_number);
            })
            .max_content_width(code_length as usize)
            .with_name("input")
            .fixed_width(code_length as usize + 1)
        );
    
    // list of all your guesses, left half of the game window
    let list_screen = Panel::new(
        ScrollView::new(ListView::new())
            .scroll_strategy(ScrollStrategy::StickToBottom)
            .with_name("list")
    )
        .fixed_height(12)
        .fixed_width(2 * (code_length as usize) + 14);
    // right half of the game window
    let sidebar_screen = LinearLayout::vertical()
        .child(ingame_settings).child(DummyView)
        .child(input_bar).child(DummyView.fixed_height(4))
        .child(Button::new("Menu", |s| { s.pop_layer(); }))
        .child(Button::new("Ragequit", |s| s.quit()));

    s.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(list_screen).child(DummyView)
                .child(sidebar_screen)
        ).title("Game")
    );
}

/// Spawns a window with the game rules
fn rules(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                // The text is separated in five parts because each has a
                // different color :[
                .child(util::text_rules_1())
                .child(util::text_rules_2())
                .child(util::text_rules_3())
                .child(util::text_rules_4())
                .child(util::text_rules_5())
        )
            .title("Rules")
            .button("Ok", |s| { s.pop_layer(); })
    );
}

/// Spawns a window asking the player if they want to quit the game
fn confirm_quit(s: &mut Cursive) {
    s.add_layer(
        OnEventView::new(
            Dialog::around(util::text_quit())
                .title("Confirm quit")
                .button("No", |s| { s.pop_layer(); })
                .button("Yes", |s| s.quit())
        // Pressing Esc again will close the window again
        ).on_event(Key::Esc, |s| { s.pop_layer(); })
    );
}