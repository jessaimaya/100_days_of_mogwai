use log::{Level};
use log::info;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use chrono::prelude::*;
use web_sys::HashChangeEvent;

use std::time::{Duration, Instant};
use std::thread::sleep;
use crate::AppView::PatchPage;
use crate::router::Route;

mod theme;
mod containers;
mod components;
mod router;

use crate::components::clock::Clock;
use crate::components::nav::{ Nav, NavModel };



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

    fn bind(&self, sub: &Subscriber<Self::ModelMsg>) {

    }

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::Mounted => {
                info!("Mounted!");
            },
            AppModel::HashChange(hash) => {
                match Route::try_from(hash.as_str()) {
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
                }
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        let c = Gizmo::from(Clock{ time: Utc::now() });
        let nav = Gizmo::from(Nav{ is_showing: false });
        let mut t_nav = nav.trns.clone();
        let mut t = trns();
        builder!{
            <section
                window:hashchange=tx.contra_filter_map(|ev:&Event| {
                    let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                    let hash = hev.new_url();
                    Some(AppModel::HashChange(hash))
                })
                patch:children=rx.branch_filter_map(AppView::patch_page)
                class="app"
            >
                {nav.view_builder()}
                <main
                    role="main"
                    class="main"
                    on:click=t.contra_map(move |_:&Event| {
                       info!("Clicking");

                        t_nav.send(&NavModel::ShowHide);
                    })
                >
                    {c.view_builder()}
                </main>
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
    let gizmo = Gizmo::from(App{route: Route::Home});

    let location_hash:String = window().location().hash().unwrap();
    gizmo.trns.send(&AppModel::HashChange(location_hash));

    let view = View::from(gizmo.view_builder());
    view.run()
}
