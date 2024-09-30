use cursive::Cursive;
use cursive::views::{EditView, ListView, ScrollView, TextView};
use fastrand;
use crate::{util::CodeError, windows};

/// Generates the code the player should guess as a `String`
pub fn get_code(code_length: u8, digit_number: u8) -> String {
    let mut result = String::new();
    for _ in 0..code_length {
        // Pushes a random digit within the specified bounds
        result = format!("{result}{}", fastrand::u8(1..=digit_number));
    }
    result
}

/// Function that runs right after the player submits their guess.
/// Depending on the validity of the guess, warns about the error or pushes the
/// guess to the list.
pub fn submit_guess(s: &mut Cursive, guess: &str, code: &str, digit_number: u8) {
    if let Err(err) = get_code_validitiy(guess, code, digit_number) {
        windows::guess_error(s, err);
        // Empties the input bar
        s.call_on_name("input", |v: &mut EditView| { v.set_content(""); });
    } else {
        s.call_on_name("list", |v: &mut ScrollView<ListView>| {
            // Calculates and formats the number of the guess in the list
            let guess_number = format!("{}.", v.get_inner().len() + 1);
            v.get_inner_mut().add_child("", TextView::new(
                format!(
                    "{:<5}{guess}{:>len_fmt$}",
                    guess_number,
                    compare_guess(guess, code),
                    len_fmt = guess.len() + 4
                )
            ));
        });

        if guess == code {
            s.call_on_name("input", |v: &mut EditView| v.disable());
            windows::win(s, code);
        }
    }
}

/// Checks whether the guess is valid in terms of length and characters.
/// Returns `Ok(())` if valid and the respective `CodeError` member otherwise.
fn get_code_validitiy(guess: &str, code: &str, digit_number: u8) -> Result<(), CodeError> {
    if guess.len() < code.len() { return Err(CodeError::Short); }
    if guess.parse::<u32>().is_err() { return Err(CodeError::NaN); }
    
    for c in guess.chars() {
        if !(1..=digit_number).contains(&(c.to_digit(10).unwrap() as u8)) {
            return Err(CodeError::DigitLimit);
        }
    }

    Ok(())
}

/// Generates the feedback consisting of `!`, `?` and `.` as a `String`
fn compare_guess(guess: &str, code: &str) -> String {
    // Shadow with a vector because it's more suited for further computation
    let mut guess: Vec<char> = guess.chars().collect();
    let mut result = String::new();

    for i in 0..code.len() {
        if code.chars().nth(i).unwrap() == guess[i] {
            guess[i] = ' ';
            // Prepends `!` to `result`
            result = format!("!{result}");
            continue;
        }
        // Checks if there's a matching character on another index, assuming it
        // doesn't belong to a "bull-pair" (`!`)
        for k in 0..code.len() {
            if code.chars().nth(i).unwrap() == guess[k]
            && code.chars().nth(k).unwrap() != guess[k] {
                guess[k] = ' ';
                result += "?";
                break;
            }
        }
    }

    // Repeatedly fills `result` with `.` until it's as long as the code
    result += &".".repeat(code.len() - result.len());
    result
}
