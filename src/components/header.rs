use mogwai::prelude::*;
use css_in_rust::*;
use crate::theme::Theme;

pub fn render_header(theme: &Theme) -> ViewBuilder<HtmlElement> {
    let style = match css_in_rust::Style::create(
        "header",
        format!("
            color: white;
            background: {};
                ", theme.background.dark),
    ) {
        Ok(style) => style,
        Err(error) => {
            panic!("An error occured while creating the style: {}", error);
        }
    };
    builder!(
        <header class=style.clone().get_class_name() >
            <a href="http://zyghost.com">
                "Schellsan's website"
            </a>
        </header>
    )
}