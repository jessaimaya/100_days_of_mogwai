use log::info;
use mogwai::prelude::*;
use crate::containers::login::LoginViewMsg;

#[derive(Clone, Debug)]
enum SignFormType {
    In,
    Up,
}

struct SignIn {}

struct SignUp {}

#[derive(Clone)]
pub enum In {
    Show(SignFormType)
}

#[derive(Clone)]
pub enum Out {
    Show(SignFormType)
}

pub struct LoginForm {
    sign_in: SignIn,
    sign_up: SignUp,
    showing: SignFormType,
}

impl Default for LoginForm {
    fn default() -> Self {
        LoginForm {
            sign_in: SignIn{},
            sign_up: SignUp{},
            showing: SignFormType::In,
        }
    }
}

impl LoginForm {
    fn get_heading(&self, tx: Transmitter<In>, rx: Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let heading = builder!{
            <h1 class="heading">
                <span
                    class={(
                        "op active",
                        rx.clone().branch_map(|Out::Show(t)| match t {
                            SignFormType::In => "active op".to_string(),
                            SignFormType::Up => "op".to_string(),
                        })
                    )}
                    on:click={tx.contra_map(|_| {
                        In::Show(SignFormType::In)
                    })}
                >
                    "Sign In"
                </span>
                <span class="middle">" or "</span>
                <span
                    class={(
                        "op",
                        rx.clone().branch_map(|Out::Show(t)| match t {
                            SignFormType::Up => "active op".to_string(),
                            SignFormType::In => "op".to_string(),
                        })
                    )}
                    on:click={tx.contra_map(|_| {
                        In::Show(SignFormType::Up)
                    })}
                >
                    "Sign Up"
                </span>
            </h1>
        };

        heading
    }

    fn get_sign_up_form(&self) -> ViewBuilder<HtmlElement> {
        builder!{<form><label>"Sign up form"</label></form>}
    }

    fn get_sign_in_form(&self) -> ViewBuilder<HtmlElement> {
        builder!{<form><label>"Sign in form"</label></form>}
    }
    /*
    fn get_form(&self, tx: Transmitter<In>, rx: Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let form_tpl = rx.clone().branch_map(|Out::Show(t)| match t {
            SignFormType::Up => self.get_sign_up_form(),
            SignFormType::In => self.get_sign_in_form(),
        });
        let form = builder!{<div>{rx.clone().branch_map(|Out::Show(t)| match t {
            SignFormType::Up => self.get_sign_up_form(),
            SignFormType::In => self.get_sign_in_form(),
        })}</div>};

        form
    }

         */
}

impl Component for LoginForm {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, sub: &Subscriber<Self::ModelMsg>) {
        match msg {
            In::Show(form_type) => {
                self.showing = form_type.clone();
                tx_view.send(&Out::Show(self.showing.clone()))
            },
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<Self::DomNode> {
        let heading = self.get_heading(tx.clone(), rx.clone());
        // let form = self.get_form(tx.clone(), rx.clone());
        builder!{
            <div class="form-content">
            {heading}
            // {form}
            </div>
        }
    }

    fn bind(&self, sub: &Subscriber<Self::ModelMsg>) {}
}