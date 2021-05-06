use log::{Level};
use log::info;
use mogwai::prelude::*;
use mogwai::component::subscriber::Subscriber;
use web_sys::{HtmlElement, HtmlInputElement};
use wasm_bindgen::{JsCast};
use js_sys::{Function};

use crate::components::login::*;
use crate::components::carousel::*;
use crate::components::login::slides::*;

pub struct Login {
    pub name: String,
    pub slides: Vec<Slide>
}

impl Default for Login {
    fn default() -> Self {
        Login {
            name: "Nombre".to_string(),
            slides: vec![
                Slide{ text: "Plan your actvities and control your progress online.".to_string(), image: "/login/rocket.png".to_string()},
                Slide{ text: "Plan your actvities 2 and control your progress online.".to_string(), image: "/login/startup.png".to_string()},
                Slide{ text: "Plan your actvities 3 and control your progress online.".to_string(), image: "/login/startup.png".to_string()},
            ]
        }
    }
}

impl Login {
    fn gen_carousel(&self) -> ViewBuilder<HtmlElement> {
        let slides = self.slides.iter().map(|s| s.get_view()).collect::<Vec<_>>();
        let carousel = Carousel {
            current_slide: 0,
            slides
        };
        builder!{<div class="info">{carousel.get_carousel()}</div>}
    }
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
        let carousel = self.gen_carousel();

        builder!{
            <div class="login">
                <div class="content">
                    {carousel}
                    <div class="form">

                    </div>
                </div>

            </div>
        }
    }
}