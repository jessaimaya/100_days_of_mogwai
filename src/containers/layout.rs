use mogwai::prelude::*;

use crate::components::{header, footer};
use crate::theme::Theme;

pub fn set_layout() -> ViewBuilder<HtmlElement> {
    let tx_click:Transmitter<Event> =Transmitter::new();
    let header = header::render_header(tx_click.clone());
    let footer = footer::render_footer(tx_click);
    builder!(
        <div class="App">
            {header}
            <main>
                <p>"Main content"</p>
            </main>
            {footer}
        </div>
    )
}