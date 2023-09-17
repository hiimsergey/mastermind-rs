fn main() {
    let mut siv = cursive::default();
    mastermind_rs::menu(&mut siv);
    siv.run();
}