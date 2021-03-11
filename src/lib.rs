use log::{Level};
use log::info;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

use std::time::{Duration, Instant};
use std::thread::sleep;

mod theme;
mod containers;
mod components;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {}

#[derive(Clone)]
enum AppModel {}

#[derive(Clone)]
enum AppView {}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, _msg: &AppModel, _tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {}

    fn view(&self, _tx: &Transmitter<AppModel>, _rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        containers::layout::set_layout()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let gizmo = Gizmo::from(App{});
    let view = View::from(gizmo.view_builder());
    view.run()
}
