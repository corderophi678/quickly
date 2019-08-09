use horrorshow::{helper::doctype, Raw};

pub fn render(html: &str, css: &str, js: &str) -> String {
    format!(
        "{}",
        html!(
        : doctype::HTML;
        html {
            head {
                link(rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/gruvbox-dark.min.css") {}
                script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                script {
                    : Raw("hljs.initHighlightingOnLoad()");
                }
                style {
                    : Raw(&css);
                }
            }
            body {
                : Raw(&html);
            }
            script {
                : Raw(&js);
            }
        }
        )
    )
}
