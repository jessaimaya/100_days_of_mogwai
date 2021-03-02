use mogwai::prelude::*;

pub fn render_footer() -> ViewBuilder<HtmlElement> {
    builder!(
        <footer>
            <h2>
                <a href="http://zyghost.com">
                    "Footer"
                </a>
            </h2>
        </footer>
    )
}