use crate::util::FormatError;
use cursive::{
    Cursive,
    theme::{ BaseColor, Color },
    views::{
        Dialog,
        EditView,
        TextView,
        ListView,
        ScrollView
    }
};
use rand::Rng;

pub fn check_guess(s: &mut Cursive, guess: &str, digit_num: u8, password: &Vec<u8>) {
    if let Err(err) = get_guess_status(guess, digit_num, password) {
        s.add_layer(
            Dialog::around(
                TextView::new(match err {
                    FormatError::DigitLimit => "The given guess contains \ndigits outside the limit.",
                    FormatError::NaN => "The given guess is \nnot a valid number.",
                    FormatError::Short => "The given guess \nis too short."
                }).style(Color::Dark(BaseColor::Red))
            ).title("Invalid input")
            .button("Ok", |s| { s.pop_layer(); })
        );
        s.call_on_name("input", |v: &mut EditView| {
            v.set_content("");
        });
    } else { compare_guess(s, guess, password); }
}

pub fn gen_password(digit_num: u8, pass_len: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::<u8>::new();
    for _ in 0..pass_len {
        pass.push(rng.gen_range(1..=digit_num));
    }
    pass
}

fn compare_guess(s: &mut Cursive, guess: &str, password: &Vec<u8>) {
    let mut feedback = String::new();
    let mut guess_vec: Vec<u8> = guess
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    for i in 0..password.len() {
        if password[i] == guess_vec[i] {
            guess_vec[i] = 0;
            feedback = format!("!{feedback}");
            continue;
        }
        for k in 0..password.len() {
            if password[i] == guess_vec[k] && guess_vec[k] != password[k] {
                guess_vec[k] = 0;
                feedback = format!("{feedback}?");
                break;
            }
        }
    }
    for _ in 0..(password.len() - feedback.len()) {
        feedback = format!("{feedback}.");
    }

    print_feedback(s, guess, feedback);
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
            return Err(FormatError::DigitLimit);
        }
    }
    Ok(())
}

fn print_feedback(s: &mut Cursive, guess: &str, feedback: String) {
    s.call_on_name("list", |v: &mut ScrollView<ListView>| {
        let len = format!("{}.", v.get_inner().len() + 1);
        v.get_inner_mut().add_child("", TextView::new(
            format!(
                "{:<5}{guess}{:>len_fmt$}",
                len,
                &feedback,
                len_fmt = guess.len() + 4
            )
        ));
        v.scroll_to_bottom();
    });

    if feedback.chars().all(|c| c == '!') {
        s.call_on_name("input", |v: &mut EditView| v.disable() );
        s.add_layer(
            Dialog::around(
                TextView::new(
                    "Congratulations!"
                ).style(Color::Dark(BaseColor::Blue))
            ).title("You won!")
            .button("Ok", |s| { s.pop_layer(); })
            .button("Menu", |s| { s.pop_layer(); s.pop_layer(); })
            .button("Quit", |s| s.quit() )
        );
    }
}
