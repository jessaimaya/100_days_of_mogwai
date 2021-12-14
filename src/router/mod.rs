#![allow(warnings)]
#![allow(unused_braces)]
use crate::components::clock::Clock;
use chrono::Utc;
use log::{trace, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

use crate::containers::login::Login;

#[cfg(feature = "weee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
use log::info;

use crate::containers::*;
use crate::router::Route::RandomMealGenerator;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    RandomMealGenerator,
}

impl TryFrom<&str> for Route {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        trace!("route try_from: {}", s);
        let hash_split = s.split("#").collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice() {
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split("/").collect::<Vec<_>>();
        trace!("route paths: {:?}", paths);
        match paths.as_slice() {
            [""] => Ok(Route::Home),
            ["", ""] => Ok(Route::Home),
            ["", "random-meal-generator"] => Ok(Route::RandomMealGenerator),
            r => Err(format!("unsupported route: {:?}", r)),
        }
    }
}

impl From<Route> for String {
    fn from(route: Route) -> String {
        match route {
            Route::Home => "#/".into(),
            Route::RandomMealGenerator => "#/random-meal-generator".into(),
        }
    }
}

impl From<&Route> for ViewBuilder<HtmlElement> {
    fn from(route: &Route) -> Self {

        let tpl = Gizmo::from(random_meal_generator::RandomMealGenerator::default()).view_builder();
        match route {
            Route::Home => tpl,
            Route::RandomMealGenerator => tpl
        }
    }
}
// return

impl From<&Route> for View<HtmlElement> {
    fn from(route: &Route) -> Self {
        ViewBuilder::from(route).into()
    }
}
