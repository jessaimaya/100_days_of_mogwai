use mogwai::prelude::*;
use chrono::prelude::*;

use crate::components::clock::Clock;
use crate::components::nav::Nav;
use crate::theme::THEME;

pub fn set_layout() -> ViewBuilder<HtmlElement> {
    let style = css_in_rust::Style::create(
        "App",
        r#"
            display: grid;
            text-align: center;
            background-color: #282c34;
            align-content: center;
        "#
    );
    let c = Gizmo::from(Clock{ time: Utc::now() });
    let nav = Gizmo::from(Nav{ is_showing: false });

    builder!(
        <section class=style.unwrap().get_class_name() >
            {nav.view_builder()}
            <main role="main">
                {c.view_builder()}
            </main>
        </section>
    )
}