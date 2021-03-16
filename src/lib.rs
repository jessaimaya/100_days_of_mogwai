use log::{Level};
use log::info;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::AppView::PatchPage;
use crate::router::Route;

mod theme;
mod containers;
mod components;
mod router;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    route: router::Route,
}

#[derive(Clone)]
enum AppModel {
    HashChange(String),
}

#[derive(Clone)]
enum AppView {
    PatchPage(Patch<View<HtmlElement>>),
    Error(String),
}

impl AppView {
    fn error(&self) -> Option<String> {
        match self {
            AppView::Error(msg) => Some(msg.clone()),
            _ => None,
        }
    }

    fn patch_page(&self) -> Option<Patch<View<HtmlElement>>> {
        match self {
            AppView::PatchPage(patch) => Some(patch.clone()),
            _ => None,
        }
    }
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::HashChange(hash) => {
                match Route::try_from(hash.as_str()) {
                    Err(msg) => tx.send(&AppView::Error(msg)),
                    Ok(route) => {
                        if route != self.route {
                            let view = View::from(ViewBuilder::from(&route));
                            self.route = route;
                            tx.send(&AppView::PatchPage(Patch::Replace {
                                index: 2,
                                value: view,
                            }));
                        }
                    }
                }
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        let style = css_in_rust::Style::create(
            "App",
            r#"
            display: grid;
            text-align: center;
            background-color: #282c34;
            align-content: center;
        "#
        );
        builder!{
            <slot
                window:hashchange=tx.contra_filter_map(|ev:&Event| {
                    let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                    let hash = hev.new_url();
                    Some(AppModel::HashChange(hash))
                })
                patch:children=rx.branch_filter_map(AppView::patch_page)
            >
                <nav>
                    <ul>
                        <li>
                            <a href=String::from(Route::Home)>"Home"</a>
                        </li>
                    </ul>
                    <li>
                        <a href=String::from(Route::Facebook)>"Facebook"</a>
                    </li>
                </nav>
                <div class=style.unwrap().get_class_name() >
                    <main>
                        {ViewBuilder::from(&self.route)}
                        <pre>{rx.branch_filter_map(AppView::error)}</pre>
                    </main>
                </div>
            </slot>
        }
        //containers::layout::set_layout()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let gizmo = Gizmo::from(App{route: Route::Home});
    let view = View::from(gizmo.view_builder());
    view.run()
}
