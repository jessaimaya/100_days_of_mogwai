#![allow(warnings)]
use serde::{Serialize, Deserialize};
use log::{Level};
use log::info;
use mogwai::prelude::*;
use reqwest::*;
use crate::utility::fetch::{fetch, Recipe};
use js_sys::JsString;

#[derive(Clone, Debug)]
pub enum In {
    Initial,
    FetchRecipe,
    FetchedRecipe(Recipe),
    DisplayRecipe(Option<Recipe>),
}
#[derive(Clone, Debug)]
pub enum Out {
    CurrentState(In),
}
#[derive(Clone, Debug)]
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

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, sub: &Subscriber<In>) {
        match msg {
            In::Initial  => {
                self.state = msg.clone();
                tx_view.send(&Out::CurrentState(self.state.to_owned()));
            },
            In::FetchRecipe => {
                self.state = msg.clone();
                sub.send_async( async {
                    let fetch_res = get_res().await;
                    In::FetchedRecipe(fetch_res)
                });
            },
            In::FetchedRecipe(recipe) => {
                self.recipe = Some(recipe.to_owned());
                self.state = In::DisplayRecipe(self.recipe.to_owned());
                tx_view.send(&Out::CurrentState(self.state.to_owned()));
            },
            In::DisplayRecipe(recipe) => {
                tx_view.send(&Out::CurrentState(self.state.to_owned()));
            }
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let new_tx = tx.clone();
        // let rec = self.recipe.to_owned();
        // info!("on view: {:?}", rec);
        builder!{
            <section class="wrapper">
                <div class="rmg" patch:children=rx.branch_map(move |Out::CurrentState(st)|
                    Patch::Replace{value: match st {
                        In::Initial => init_view(&new_tx),
                        In::FetchRecipe => fetching_view(&new_tx),
                        In::DisplayRecipe(recipe) => {
                            info!("patching to displayrecipe : {:?}", recipe);
                            display_recipe(&new_tx, recipe.to_owned())
                        },
                        _ => fetching_view(&new_tx),
                    }, index: 0}.clone()
                )>
                    {
                        init_view(tx)
                    }
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
        </div>
    }
}

fn display_recipe(tx:&Transmitter<In>, recipe_resp: Option<Recipe>) -> ViewBuilder<HtmlElement> {
    info!("displaying recipe: {:?}", recipe_resp);
    match recipe_resp {
        Some(recipe) => {
            let recipe = &recipe.meals[0];
            builder!(
                     <div class="rmg__fold">
                        <button
                            class="refresh"
                            on:click=tx.contra_map(|_| In::FetchRecipe)
                        >
                            <i class="fa fa-random"></i>
                        </button>
            <section class="recipe">
                <div class="recipe__image">
                    <img src={&recipe.str_meal_thumb} alt={&recipe.str_meal} />
                </div>
                <div class="recipe__content">
                    <div class="recipe__content-header">
                        <div class="recipe__content-category">
                            <p><i class="fa fa-tag" aria-hidden="true"></i>{&recipe.str_category}</p>
                        </div>
                        <div class="recipe__content-region">
                                <p><i class="fas fa-globe" aria-hidden="true"></i>{&recipe.str_area}</p>
                        </div>
                    </div>
                    <div class="recipe__content-name"><h1>{&recipe.str_meal}</h1></div>
                    <div class="recipe__content-ingredients">
                        <ul class="recipe__content-list">
                            {if let Some(item) = get_ingredient_tpl(recipe.str_ingredient1.to_owned(), recipe.str_measure1.to_owned()){
                               item
                            } else {builder!(<li></li>)}}
                            {if let Some(item) = get_ingredient_tpl(recipe.str_ingredient2.to_owned(), recipe.str_measure2.to_owned()){
                               item
                            } else {builder!(<li></li>)}}{if let Some(item) = get_ingredient_tpl(recipe.str_ingredient3.to_owned(), recipe.str_measure3.to_owned()){
                               item
                            } else {builder!(<li></li>)}}{if let Some(item) = get_ingredient_tpl(recipe.str_ingredient4.to_owned(), recipe.str_measure4.to_owned()){
                               item
                            } else {builder!(<li></li>)}}
                        </ul>
                    </div>
                    <div class="recipe__content-source">
                        <a title={&recipe.str_meal} href={&recipe.str_source} target="_blank">"View Recipe"</a>
                    </div>
                </div>
            </section>
        </div>
            )
        },
        None => builder!(<div>"none"</div>)
    }
}

async fn get_res() -> Recipe {
    info!("Calling get_res");
    //let url = "https://jsonplaceholder.typicode.com/posts/1".to_string();
    let url = "https://www.themealdb.com/api/json/v1/1/random.php".to_string();
    let my_resp = fetch(url, "Get".to_string()).await;
    // info!("Obj: {:?}", my_resp);
    my_resp.expect("couldn't serialize to an Obj").clone()
}

fn get_ingredient_tpl(ingredient: Option<String>, measure: Option<String>) -> Option<ViewBuilder<HtmlElement>> {
    if let (Some(ing), Some(measu)) = (ingredient, measure) {
        return Some(builder!(
                                <li>
                                        <span class="title">{format!("{}: ",ing)}</span>
                                       <span class="measure">{measu}</span>
                                </li>
        ));
    }
    None
}