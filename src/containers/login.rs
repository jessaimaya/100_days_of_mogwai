use log::{Level};
use log::info;
use mogwai::prelude::*;
use mogwai::component::subscriber::Subscriber;
use web_sys::{HtmlElement, HtmlInputElement};
use wasm_bindgen::{JsCast};
use js_sys::{Function};

use crate::components::login::*;
use crate::components::carousel::*;

pub struct Login {
    pub name: String,
}

#[derive(Clone)]
pub enum LoginModelMsg {
    NameEditing(String),
    Init
}

#[derive(Clone)]
pub enum LoginViewMsg {
    Name(String)
}

impl Component for Login {
    type ModelMsg = LoginModelMsg;
    type ViewMsg = LoginViewMsg;
    type DomNode = HtmlElement;

    fn bind(&self, input_sub: &Subscriber<Self::ModelMsg>) {
        info!("Something happened");
    }

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, sub: &Subscriber<Self::ModelMsg>) {
        match msg {
            LoginModelMsg::Init => {
                info!("Init!");
                let elem = document().get_element_by_id("name").unwrap();
                let input: &HtmlElement = elem.unchecked_ref();
                input.focus();
            },
            LoginModelMsg::NameEditing(inputText) => {
                info!("editing...");
                self.name = inputText.clone();
                tx_view.send(&LoginViewMsg::Name(self.name.clone()));
            },
        }
    }

    fn view(&self, tx: &Transmitter<Self::ModelMsg>, rx: &Receiver<Self::ViewMsg>) -> ViewBuilder<Self::DomNode> {
        let carousel = Gizmo::from(Carousel {
            current_slide: 0,
            slides: vec![Slide{color: "#f00".to_string()}, Slide{color: "#0f0".to_string()}]
        });

        builder!{
            <div class="login">
                <div class="content">
                    <div class="info">
                    {carousel.view_builder()}
                </div>
                    <div class="form">

                    </div>
                </div>

            </div>
        }
    }
}