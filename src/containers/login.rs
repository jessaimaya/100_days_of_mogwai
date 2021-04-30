use log::{Level};
use log::info;
use mogwai::prelude::*;
use mogwai::component::subscriber::Subscriber;
use web_sys::{HtmlElement, HtmlInputElement};
use wasm_bindgen::{JsCast};
use js_sys::{Function};

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
        info!("almost rendered");
        builder!{
            <div>
                <h1>"Login"</h1>
                <input
                    cast:type=web_sys::HtmlInputElement
                    id="name"
                    type="text"
                    autofocus="autofocuss"
                    on:load = tx.contra_map(|_: &Event| {
                        info!("input loaded!");
                        LoginModelMsg::Init
                    })
                    // post:build = tx.contra_map(|el: &HtmlInputElement| {
                       // info!("what is this: {:?}", el);
                        // ModelMsg::NameEditing(el.clone())
                    // })
                    on:keyup= tx.contra_map(|e: &Event| {
                        let target = e.target().unwrap();
                        let input: &HtmlInputElement = target.unchecked_ref();

                        info!("what is this: {:?}", input.value());
                        LoginModelMsg::NameEditing(input.value())
                    })
                />
            </div>
        }
    }
}