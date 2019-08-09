use super::{Content, Header};
use gtk;
use gtk::prelude::*;

pub struct App {
    pub window: gtk::Window,
    pub header: Header,
    pub content: Content,
}

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("Error Initializing GTK Application");
            std::process::exit(1);
        }
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("Quicky");
        window.set_default_size(800, 600);
        gtk::Window::set_default_icon_name("iconname");
        window.add(&content.container);

        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }

    pub fn connect_events(self) -> ConnectedApp {
        ConnectedApp(self)
    }
}

pub struct ConnectedApp(App);
impl ConnectedApp {
    pub fn then_execute(&self) {
        self.0.window.show_all();
        gtk::main();
    }
}
