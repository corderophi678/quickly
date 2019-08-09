#[macro_use]
extern crate horrorshow;

pub mod ui;

use ui::App;

fn main() {
    App::new().connect_events().then_execute();
}
