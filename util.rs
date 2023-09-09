use cursive::views::{
    Button,
    Dialog
};
use crate::menu;
use webbrowser;

pub fn source_button(url: &'static str) -> Button {
    Button::new("source ↗", |s| {
        if !webbrowser::open(url).is_ok() {
            s.clear_global_callbacks('q');
            s.add_global_callback('q', |s| {
                s.pop_layer();
                s.add_global_callback('q', menu);
            });
            s.add_layer(Dialog::info(
"Couldn't connect to the website. :[
Check that you have a web browser installed."
            ).title("Connection error")
            );
        }
    })
}
