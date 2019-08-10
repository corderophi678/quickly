use gtk::*;
use std::path::PathBuf;

pub struct OpenDialog(FileChooserDialog);

impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );

        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        path.map(|p| open_dialog.set_current_folder(p));

        OpenDialog(open_dialog)
    }
    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
        let save_dialog = FileChooserDialog::new(
            Some("Save As"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }
    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}
impl Drop for SaveDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}
