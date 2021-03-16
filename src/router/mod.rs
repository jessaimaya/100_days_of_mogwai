#![allow(unused_braces)]
use log::{trace, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;
use crate::components::clock::Clock;
use chrono::Utc;
#[cfg(feature = "weee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Home,
    Facebook,
    Settings,
}

impl TryFrom<&str> for Route {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        trace!("route try_from: {}", s);
        let hash_split = s.split("#").collect::<Vec<_>>();
        let after_hash = match hash_split.as_slice(){
            [_, after] => Ok(after),
            _ => Err(format!("route must have a hash: {}", s)),
        }?;

        let paths: Vec<&str> = after_hash.split("/").collect::<Vec<_>>();
        trace!("route paths: {:?}", paths);

        match paths.as_slice() {
            [""] => Ok(Route::Home),
            ["", ""] => Ok(Route::Home),
            ["", "facebook"] => Ok(Route::Facebook),
            ["", "settings"] => Ok(Route::Settings),
            r => Err(format!("unsupported route: {:?}", r)),
        }
    }
}

impl From<Route> for String {
    fn from(route: Route) -> String {
        match route {
            Route::Home => "#/".into(),
            Route::Facebook => "#/facebook".into(),
            Route::Settings=> "#/settings".into(),
        }
    }
}

impl From<&Route> for ViewBuilder<HtmlElement> {
    fn from(route: &Route) -> Self {
        match route {
            Route::Home => {
                let c = Gizmo::from(Clock{time: Utc::now()});
                return builder!{
                    <main>
                        {c.view_builder()}
                    </main>
                }
            },
            Route::Facebook => builder!{<div><h1>"Facebook"</h1></div>},
            Route::Settings => builder!{<div><h1>"Settings"</h1></div>},
        }
    }
}

impl From<&Route> for View<HtmlElement> {
    fn from(route: &Route) -> Self {
        ViewBuilder::from(route).into()
    }
}