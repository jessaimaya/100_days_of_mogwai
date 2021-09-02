#![allow(warnings)]
use chrono::prelude::*;
use log::info;
use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

use crate::router::Route;
use crate::AppView::PatchPage;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod components;
mod containers;
mod router;
mod theme;
mod utility;

use crate::components::nav::nav_view;

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
    Mounted,
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

    fn bind(&self, sub: &Subscriber<Self::ModelMsg>) {}

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::Mounted => {
                info!("Mounted!");
            }
            AppModel::HashChange(hash) => match Route::try_from(hash.as_str()) {
                Err(msg) => tx.send(&AppView::Error(msg)),
                Ok(route) => {
                    if route != self.route {
                        let view = View::from(ViewBuilder::from(&route));
                        self.route = route;
                        tx.send(&AppView::PatchPage(Patch::Replace {
                            index: 1,
                            value: view,
                        }));
                    }
                }
            },
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        builder! {
            <section
                window:hashchange=tx.contra_filter_map(|ev:&Event| {
                    let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                    let hash = hev.new_url();
                    Some(AppModel::HashChange(hash))
                })
                patch:children=rx.branch_filter_map(AppView::patch_page)
                class="app"
            >
            {nav_view()}
            <main>{ViewBuilder::from(&self.route)}</main>
            </section>
        }
        //containers::layout::set_layout()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();
    info!("is printing!");
    let gizmo = Gizmo::from(App { route: Route::Home });

    let location_hash: String = window().location().hash().unwrap();
    gizmo.trns.send(&AppModel::HashChange(location_hash));

    let view = View::from(gizmo.view_builder());
    view.run()
}
