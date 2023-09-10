fn main() {
    let mut siv = cursive::default();
    mastermind_rs::menu(&mut siv);
    siv.run();
    // TODO recheck every single button
        // for pop.layers
        // for Button::new
        // for every global callback
}