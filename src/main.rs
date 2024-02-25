// for now RENDOC does one file at a time
// TODO: add support for multiple files
// TODO: templates; e.g. docs == ['README.md', 'ARCHITECTURE.md']

use comrak::{markdown_to_html, ComrakOptions};
use maud::{html, Markup, PreEscaped, DOCTYPE};

fn render_to_markup(file: &str) -> Markup {
    let markdown_input = std::fs::read_to_string(file).unwrap();
    let html_output = markdown_to_html(&markdown_input, &ComrakOptions::default());
    html! {
        (PreEscaped(html_output))
    }
}

fn style() -> Markup {
    html! {
        style {
r#"
body {
    font-family: sans-serif;
    color: #333;
    background-color: #f8f8f8;
    margin: auto;
    max-width: 70%;
}
"#
        }
    }
}

fn template(markup: Markup, title: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                (style())
            }
            body {
                (markup)
            }
        }
    }
}

fn stub(path: &str) -> String {
    path.split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string()
}

fn save_markup_to_file(markup: Markup, file: &str) {
    let title = stub(file);
    let html = template(markup, &title);
    let html = html.into_string();

    std::fs::write(format!("{}.html", title), html).unwrap();
}

fn main() {
    if std::env::args().len() != 2 {
        println!("Usage: rendoc <file>");
        std::process::exit(1);
    }

    let file = std::env::args().nth(1).unwrap();
    let markup = render_to_markup(&file);
    save_markup_to_file(markup, &file);
}
