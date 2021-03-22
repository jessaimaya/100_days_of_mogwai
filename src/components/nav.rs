use log::info;
use mogwai::prelude::*;
use mogwai::component::subscriber::Subscriber;
use web_sys::HtmlElement;

use crate::router;

#[derive(Clone)]
pub enum NavModel {
    ShowHide,
}
#[derive(Clone)]
pub enum NavView {
    ShowHide(bool),
}
pub struct Nav {
    pub is_showing: bool,
}

impl Component for Nav {
    type ModelMsg = NavModel;
    type ViewMsg = NavView;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, sub: &Subscriber<Self::ModelMsg>) {
        match msg {
            NavModel::ShowHide => {
                self.is_showing = !self.is_showing;
                tx_view.send(&NavView::ShowHide(self.is_showing));
            }
        }
    }

    fn view(&self, tx: &Transmitter<Self::ModelMsg>, rx: &Receiver<Self::ViewMsg>) -> ViewBuilder<Self::DomNode> {
        let css_open = rx.branch_map(|msg| {
           match msg {
               NavView::ShowHide(flag) => match *flag {
                   true => format!("0"),
                   false => format!("-240px")
               }
           }
        });
        builder!(
        <div>
                <button
                    style=vec![
                        "position: fixed;",
                        "z-index: 1;",
                        "right: 0;",
                        "top: 0;",
                    ].concat()
                    on:click = tx.contra_map(|_| NavModel::ShowHide )>
                    "Menu"
                </button>
                <nav
                    class="cbp-spmenu cbp-spmenu-vertical cbp-spmenu-left"
                    id="cbp-spmenu-s1"
                    style:left = ("-240px", css_open)
                >
                    <h3>"Menu"</h3>
                    <a href=String::from(router::Route::Home)>"Home"</a>
                    <a href=String::from(router::Route::Login)>"Login"</a>
                </nav>
        </div>
        )
    }
}