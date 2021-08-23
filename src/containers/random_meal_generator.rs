#![allow(warnings)]
use log::{Level};
use log::info;
use mogwai::prelude::*;

#[derive(Clone)]
pub struct Recipe {}

#[derive(Clone)]
pub enum In {
    Initial,
    FetchRecipe,
    DisplayRecipe,
}
#[derive(Clone)]
pub enum Out {
    CurrentState(In),
}
#[derive(Clone)]
pub struct RandomMealGenerator {
    recipe: Option<Recipe>,
    state: In
}

impl Default for RandomMealGenerator {
    fn default() -> Self {
        RandomMealGenerator{
            recipe: None,
            state: In::Initial,
        }
    }
}

impl Component for RandomMealGenerator {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, _sub: &Subscriber<In>) {
        self.state = msg.clone();
        match msg {
            In::Initial  => {
                tx_view.send(&Out::CurrentState(self.state.to_owned()));
            },
            In::FetchRecipe => {
                tx_view.send(&Out::CurrentState(self.state.to_owned()));
            },
            In::DisplayRecipe => {
                tx_view.send(&Out::CurrentState(self.state.to_owned()));
            }
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let new_tx = tx.clone();
        builder!{
            <section class="wrapper">
                <div class="rmg" patch:children=rx.branch_map(move |Out::CurrentState(st)|
                    Patch::Replace{value: match st {
                        In::Initial => init_view(&new_tx),
                        In::FetchRecipe => fetching_view(&new_tx),
                        In::DisplayRecipe => display_recipe(&new_tx)
                    }, index: 0}.clone()
                )>
                    {init_view(&tx)}
                </div>
                <div class="footnotes">
                    <a class="footnotes__note" target="_blank" href="https://www.freepik.com/vectors/food">"Image: Food vector created by jcomp - www.freepik.com"</a>
                    <a class="footnotes__note" target="_blank" href="https://dribbble.com/shots/15300407-Cooking-app">"Design: Recipe finder app with step-by-step detailed cooking"</a>
                </div>
            </section>
        }
    }
}

fn init_view(tx:&Transmitter<In>) -> ViewBuilder<HtmlElement> {
    builder!{
       <div class="rmg__fold">
            <h2 class="rmg__fold-title">"Feeling hungry?"</h2>
            <p class="rmg__fold-subtitle">"Get a random meal by clicking  below"</p>
            <img class="rmg__fold-img" src="./images/rmg/cooking.svg" alt="People cooking a meal"/>
            <button
                class="rmg__fold-btn"
                on:click=tx.contra_map(|_| In::FetchRecipe)
            >
                "Get started"
            </button>
        </div>
    }
}

fn fetching_view(tx:&Transmitter<In>) -> ViewBuilder<HtmlElement> {
    builder!{
        <div class="rmg__fold">
            <div class="sk-chase">
                <div class="sk-chase-dot"></div>
                <div class="sk-chase-dot"></div>
                <div class="sk-chase-dot"></div>
                <div class="sk-chase-dot"></div>
                <div class="sk-chase-dot"></div>
                <div class="sk-chase-dot"></div>
            </div>
            <p class="rmg__fold-loading">"Fetching recipe..."</p>
            <button
                class="rmg__fold-btn"
                on:click=tx.contra_map(|_| In::DisplayRecipe)
            >
                "Continue"
            </button>
        </div>
    }
}

fn display_recipe(tx:&Transmitter<In>) -> ViewBuilder<HtmlElement> {
    builder!{
        <div>
            <h1>"Recipe"</h1>
        <button
                class="rmg__fold-btn"
                on:click=tx.contra_map(|_| In::Initial)
            >
                "Init!"
            </button>
        </div>
    }
}