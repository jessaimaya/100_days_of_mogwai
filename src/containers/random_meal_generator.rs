#![allow(warnings)]
use log::{Level};
use log::info;
use mogwai::prelude::*;

#[derive(Clone)]
pub enum AppState {
    Init,
    Fetch,
    Render
}

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
    CurrentState(AppState),
    // PatchItem(Patch<ViewBuilder<HtmlElement>>),
}
#[derive(Clone)]
pub struct RandomMealGenerator {
    recipe: Option<Recipe>,
    state: AppState
}

impl Default for RandomMealGenerator {
    fn default() -> Self {
        RandomMealGenerator{
            recipe: None,
            state: AppState::Init,
        }
    }
}

impl Component for RandomMealGenerator {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, sub: &Subscriber<In>) {
        match msg {
            In::Initial => {
                self.state = AppState::Init;
                tx_view.send(&Out::CurrentState(self.state.clone()));
            },
            In::FetchRecipe => {
                // tx_view.send(&Out::PatchItem(Patch::Replace { value: fetching_view(), index:0 }));
                self.state = AppState::Fetch;
                tx_view.send(&Out::CurrentState(self.state.clone()));
            },
            In::DisplayRecipe => {
                //tx_view.send(&Out::PatchItem(Patch::Replace {value: display_recipe(), index:0 }))
                self.state = AppState::Render;
                tx_view.send(&Out::CurrentState(self.state.clone()));
            }
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let new_tx = tx.clone();
        builder!{
            <section class="wrapper">
                <div class="rmg" patch:children=rx.branch_map(move |Out::CurrentState(st)| {
                    match st {
                        AppState::Init => {
                            let patch = Patch::Replace{value: init_view(&new_tx), index: 0};
                            patch.clone()
                        },
                        AppState::Fetch => {
                            let patch = Patch::Replace{value: fetching_view(&new_tx), index: 0};
                            patch.clone()
                        },
                        AppState::Render => {
                            let patch = Patch::Replace{value: display_recipe(&new_tx), index: 0};
                            patch.clone()
                        }
                    }
                })>
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