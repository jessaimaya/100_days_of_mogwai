use log::{Level, Record};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
// use css_in_rust::style;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    clicks: u32,
    style: css_in_rust::Style,
}

#[derive(Clone)]
enum AppModel {
    Click,
}

#[derive(Clone)]
enum AppView {
    Clicked(u32),
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::Click => {
                self.clicks += 1;
                tx.send(&AppView::Clicked(self.clicks));
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        builder! {
            <div
                class=self.style.clone().get_class_name()
                on:click=tx.contra_map(|_| AppModel::Click)>
                <p class="my-color">
                    {(
                        "Hello from mogwai!",
                        rx.branch_map(|msg| {
                            match msg {
                                AppView::Clicked(1) => format!("Caught 1 click, click again 😀"),
                                AppView::Clicked(n) => format!("Caught {} clicks", n),
                            }
                        })
                    )}
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let style = match css_in_rust::Style::create(
        "Component", // The class prefix
        // The actual css
        r#"
            color: white;
                "#,
    ) {
        Ok(style) => style,
        Err(error) => {
            panic!("An error occured while creating the style: {}", error);
        }
    };
    let gizmo = Gizmo::from(App{ clicks: 0, style });
    let view = View::from(gizmo.view_builder());
    view.run()
}
