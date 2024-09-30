mod logic; mod util; mod windows;

fn main() {
    let mut siv = cursive::default();
    windows::menu(&mut siv);
    siv.run();
}
