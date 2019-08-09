use super::content::Lang;
use gtk::*;

pub struct Source {
    pub container: gtk::ScrolledWindow,
    pub view: sourceview::View,
    pub buff: sourceview::Buffer,
    pub lang: Lang,
}

impl Source {
    pub fn new(lang: Lang) -> Source {
        let buff = sourceview::Buffer::new(None);
        let view = sourceview::View::new_with_buffer(&buff);
        let container = gtk::ScrolledWindow::new(None, None);
        match lang {
            Lang::Css => {
                buff.set_text("css");
            }
            Lang::Html => {
                buff.set_text("css");
            }
            Lang::Js => {
                buff.set_text("css");
            }
        }

        container.add(&view);

        configure_source_view(&lang, &view, &buff);

        Source {
            container,
            view,
            buff,
            lang,
        }
    }
}

fn configure_source_view(lang: &Lang, view: &sourceview::View, buff: &sourceview::Buffer) {
    use sourceview::*;
    match lang {
        Lang::Css => {
            LanguageManager::new()
                .get_language("css")
                .map(|css| buff.set_language(Some(&css)));
        }
        Lang::Html => {
            LanguageManager::new()
                .get_language("html")
                .map(|html| buff.set_language(Some(&html)));
        }
        Lang::Js => {
            LanguageManager::new()
                .get_language("javascript")
                .map(|js| buff.set_language(Some(&js)));
        }
    }

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("Builder")
        .or(manager.get_scheme("Classic"))
        .map(|theme| buff.set_style_scheme(Some(&theme)));

    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_insert_spaces_instead_of_tabs(true);
    view.set_indent_width(2);
    view.set_smart_backspace(true);
    view.set_right_margin(100);
    view.set_left_margin(10);
    view.set_top_margin(10);
    view.set_bottom_margin(10);
    view.set_show_right_margin(true);
    view.set_background_pattern(BackgroundPatternType::None);
}
