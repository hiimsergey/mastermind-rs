// TODO add confimation screen when pressing Esc
use crate::util::FormatError;
use cursive::{
    Cursive,
    theme::{
        BaseColor,
        Color
    },
    utils::markup::StyledString,
    views::{
        Dialog,
        TextView,
        ListView
    }
};
use rand::Rng;

// TODO clear input after submit, no matter if valid
pub fn check_guess(s: &mut Cursive, guess: &str, digit_num: u8, password: &Vec<u8>) {
    if let Err(err) = get_guess_status(guess, digit_num, password) {
        s.add_layer(Dialog::around(TextView::new(match err {
            FormatError::BeyondDigitBound => "The given guess contains \ndigits outside the limit.",
            FormatError::NaN => "The given guess is \nnot a valid number.",
            FormatError::Short => "The given guess \nis too short."
        }))
            .title("Invalid input")
            .button("Ok", |s| { s.pop_layer(); })
        );
    } else { print_feedback(s, guess, password); }
}

fn get_guess_status(guess: &str, digit_num: u8, password: &Vec<u8>) -> Result<(), FormatError> {
    if guess.len() < password.len() {
        return Err(FormatError::Short);
    }

    if let Err(_) = guess.parse::<u32>() {
        return Err(FormatError::NaN);
    }

    for c in guess.chars() {
        if !(1..=digit_num).contains(&(c.to_digit(10).unwrap() as u8)) {
            return Err(FormatError::BeyondDigitBound);
        }
    }

    Ok(())
}

pub fn gen_password(digit_num: u8, pass_len: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::<u8>::new();
    for _ in 0..pass_len {
        pass.push(rng.gen_range(1..=digit_num));
    }
    pass
}

fn print_feedback(s: &mut Cursive, guess: &str, password: &Vec<u8>) {
    // TODO write win message
    // two buttons: ok - pop_layer; menu - menu
    // before the window appears, the input bar disappears
    
    s.call_on_name("list", |v: &mut ListView| {
        // TODO print round number in the label
        v.add_child("123.", TextView::new(guess));
    });

    // SORT feedback by !-?-.
    let mut feedback = String::new();
    let mut guess: Vec<u8> = guess
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    for i in 0..password.len() {
        if password[i] == guess[i] {
            guess[i] = 0;
            feedback = format!("!{feedback}");
            continue;
        }
        for k in 0..password.len() {
            if password[i] == guess[k] && guess[k] != password[k] {
                guess[k] = 0;
                feedback = format!("{feedback}?");
                break;
            }
        }
    }
    for _ in 0..(password.len() - feedback.len()) {
        feedback = format!("{feedback}.");
    }
    
    s.call_on_name("feedback", |v: &mut ListView| {
        v.add_child("", TextView::new(&feedback));
    });

    if feedback.chars().all(|c| c == '!') {
        s.add_layer(Dialog::info(StyledString::styled("Congratulations!", Color::Dark(BaseColor::Blue)))
            .title("You won!")
            .button("Menu", |s| { s.pop_layer(); s.pop_layer(); })
            .button("Quit", |s| s.quit() )
        );
    }
}