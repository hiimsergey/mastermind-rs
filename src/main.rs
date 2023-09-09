fn main() {
    let mut siv = cursive::default();
    mastermind_rs::menu(&mut siv);
    siv.run();
    // TODO change theme
    // TODO today
    // write rules page
    // write game settings util
    // write game settings as far as you can
}
