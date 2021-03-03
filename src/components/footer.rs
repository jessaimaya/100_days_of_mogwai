use mogwai::prelude::*;

pub fn render_footer(tx: Transmitter<Event>) -> ViewBuilder<HtmlElement> {
    builder!(
        <footer>
            <h2 on:click= tx.clone() >
               "Click me"
            </h2>
        </footer>
    )
}