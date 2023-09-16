fn main() {
    let mut siv = cursive::default();
    mastermind_rs::menu(&mut siv);
    siv.run();
    // TODO recheck every single button
        // for pop.layers
        // for Button::new
        // for every global callback
    // TODO recheck every single Dialog
        // change it to Dialog if necessary
    // TODO add Readme to GitHub
    // TODO search for print or eprints
}