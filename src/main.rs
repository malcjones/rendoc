use maud::{html, Markup, PreEscaped, DOCTYPE};
use pulldown_cmark::{html, Options, Parser};

fn render_to_markup(file: &str) -> Markup {
    let markdown_input = std::fs::read_to_string(file).unwrap();
    html! { (PreEscaped(render_markdown(&markdown_input))) }
}

fn style() -> Markup {
    html! {
        style {
r#"
a {
    color: #268bd2; /* Solarized blue */
}

a:visited {
    color: #6c71c4; /* Solarized violet */
}

body {
    font-family: sans-serif;
    color: #073642; /* Solarized base02 for higher contrast */
    background-color: #fdf6e3; /* Solarized base3 */
    margin: auto;
    max-width: 70%;
}

pre {
    font-family: 'Courier New', Courier, monospace; /* A common monospace font */
    background-color: #eee8d5; /* Solarized base2 */
    color: #586e75; /* Solarized base01 for better contrast in pre blocks */
    padding: 10px;
    border-left: 5px solid #93a1a1; /* Solarized base1 */
    overflow: auto;
    border-radius: 4px; /* Rounded corners */
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.05); /* Slight inner shadow */
}

@media (prefers-color-scheme: dark) {
    a {
        color: #268bd2; /* Solarized blue */
    }

    a:visited {
        color: #6c71c4; /* Solarized violet */
    }

    body {
        color: #839496; /* Solarized base01 */
        background-color: #002b36; /* Solarized base03 */
    }

    pre {
        background-color: #073642; /* Solarized base02 */
        color: #93a1a1; /* Solarized base1 */
        border-left: 5px solid #586e75; /* Solarized base01 */
    }
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
            body { (markup) }
        }
    }
}

fn save_markup_to_file(markup: Markup, file: &str) {
    let title = file.rsplit('/').next().unwrap().split('.').next().unwrap();
    let html = template(markup, title).into_string();
    std::fs::write(format!("{}.html", title), html).unwrap();
}

fn render_markdown(input: &str) -> String {
    let parser = Parser::new_ext(input, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn main() {
    let file = std::env::args().nth(1).expect("Usage: rendoc <file>");
    save_markup_to_file(render_to_markup(&file), &file);
}
