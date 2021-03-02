use mogwai::prelude::*;

use crate::components::{header, footer};
use crate::theme::Theme;

pub fn set_layout() -> ViewBuilder<HtmlElement> {
    let header = header::render_header();
    let footer = footer::render_footer();
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