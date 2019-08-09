use gtk::prelude::*;

pub struct Header {
    pub container: gtk::HeaderBar,
    pub open: gtk::Button,
    pub save: gtk::Button,
    pub save_as: gtk::Button,
}
impl Header {
    pub fn new() -> Header {
        let container = gtk::HeaderBar::new();

        container.set_title(Some("Quicky"));
        container.set_show_close_button(true);

        let open = gtk::Button::new_with_mnemonic("_Open");
        let save = gtk::Button::new_with_mnemonic("_Save");
        let save_as = gtk::Button::new_with_mnemonic("Save _As");

        Header {
            container,
            open,
            save,
            save_as,
        }
    }
}
