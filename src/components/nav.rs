use mogwai::prelude::*;
use web_sys::HtmlElement;

pub fn nav_view() -> ViewBuilder<HtmlElement> {
    let view_builder:ViewBuilder<HtmlElement> = builder!{
        <header class="navbar">
            <section class="navbar-section">
                <a href="#/" title="Home" class="logo">
                    <img src="./images/gizmo.svg" alt="gizmo | mogwai" class="logo__img"/>
                </a>
            </section>
            <section class="navbar-center">
                <h1 class="title">
                    "100 days of Mogwai"
                </h1>
            </section>
            <section class="navbar-section"></section>
        </header>
    };
    view_builder
}