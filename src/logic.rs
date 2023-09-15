// TODO add confimation screen when pressing Esc
use crate::util::FormatError;
use cursive::{
    Cursive,
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
            FormatError::BeyondDigitBound => "The given guess contains digits outside the limit.",
            FormatError::NaN => "The given guess is not a valid number.",
            FormatError::Short => "The given guess is too short."
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

                                        // TODO are u sure about that
pub fn gen_password(digit_num: u8, pass_len: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut pass = Vec::<u8>::new();
    for _ in 0..pass_len {
        pass.push(rng.gen_range(1..=digit_num));
    }
    pass
}

fn print_feedback(s: &mut Cursive, guess: &str, password: &Vec<u8>) {
    // TODO guide
    s.call_on_name("list", |v: &mut ListView| {
        v.add_child("123.", TextView::new("1234"));
    });
    s.call_on_name("feedback", |v: &mut ListView| {
        v.add_child("", TextView::new("!??_"));
    });
}