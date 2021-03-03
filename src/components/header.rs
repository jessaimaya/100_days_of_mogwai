use mogwai::prelude::*;
use css_in_rust::*;

use crate::theme::Theme;

pub fn render_header(tx_click: Transmitter<Event>) -> ViewBuilder<HtmlElement> {
    let rx_theme = Receiver::<String>::new();
    tx_click.wire_fold(
        &rx_theme,
        false, // the intial value for is_red
        |is_light, _| {
            let out = if *is_light { "light".into() } else { "dark".into() };
            *is_light = !*is_light;
            out
        },
    );
    let style = match css_in_rust::Style::create(
        "header",
        format!("
            color: white;
            background: {};
                ",  Theme.background.light),
    ) {
        Ok(style) => style,
        Err(error) => {
            panic!("An error occured while creating the style: {}", error);
        }
    };
    builder!(
        <header class=style.clone().get_class_name() >
            <a href="http://zyghost.com">
                {("Schellsan's website", rx_theme.branch())}
            </a>
        </header>
    )
}