use gtk::prelude::*;

pub fn get_buffer(buffer: &sourceview::Buffer) -> Option<String> {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true)
}
