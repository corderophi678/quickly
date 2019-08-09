use super::super::preview::render;
use super::misc::*;
use super::source::Source;
use gtk::prelude::*;
use webkit2gtk::*;

#[derive(Clone)]
pub enum Lang {
    Css,
    Html,
    Js,
}
#[derive(Clone)]
pub struct Content {
    pub container: gtk::Paned,
    pub source: gtk::Box,
    pub preview: webkit2gtk::WebView,
    pub css: Source,
    pub html: Source,
    pub js: Source,
}
impl Content {
    pub fn new() -> Content {
        let container = gtk::Paned::new(gtk::Orientation::Horizontal);

        let source = gtk::Box::new(gtk::Orientation::Vertical, 2);
        let css_label = gtk::Label::new("CSS");
        let css = Source::new(Lang::Css);
        let html_label = gtk::Label::new("HTML");
        let html = Source::new(Lang::Html);
        let js_label = gtk::Label::new("JavaScript");
        let js = Source::new(Lang::Js);

        source.pack_start(&html_label, false, false, 0);
        source.pack_start(&html.container, true, true, 0);
        source.pack_start(&css_label, false, false, 0);
        source.pack_start(&css.container, true, true, 0);
        source.pack_start(&js_label, false, false, 0);
        source.pack_start(&js.container, true, true, 0);

        container.pack1(&source, true, true);

        let context = webkit2gtk::WebContext::get_default().unwrap();
        let preview = webkit2gtk::WebView::new_with_context(&context);

        container.pack2(&preview, true, true);

        source.set_size_request(100, -1);
        preview.set_size_request(100, -1);

        Content {
            container,
            source,
            preview,
            css,
            html,
            js,
        }
    }
    pub fn render_preview(&self) {
        let css = get_buffer(&self.css.buff.clone()).unwrap_or("".to_string());
        let html = get_buffer(&self.html.buff.clone()).unwrap_or("".to_string());
        let js = get_buffer(&self.js.buff.clone()).unwrap_or("".to_string());
        self.preview.load_html(&render(&html, &css, &js), None);
    }
}
