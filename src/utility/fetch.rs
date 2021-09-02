use log::{Level};
use log::info;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use std::fmt::Debug;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    pub meals: Vec<Meal>
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub str_meal: String,
    pub str_category: String,
    pub str_area: String,
    pub str_meal_thumb: String,
    pub str_source: String,
    pub str_ingredient1: Option<String>,
    pub str_ingredient2: Option<String>,
    pub str_ingredient3: Option<String>,
    pub str_ingredient4: Option<String>,
    pub str_ingredient5: Option<String>,
    pub str_measure1: Option<String>,
    pub str_measure2: Option<String>,
    pub str_measure3: Option<String>,
    pub str_measure4: Option<String>,
    pub str_measure5: Option<String>,
}

impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            meals: vec![Meal {
                str_area: "British".to_string(),
                str_category: "Beef".to_string(),
                str_meal: "British beef".to_string(),
                str_source: "https://www.bbcgoodfood.com/recipes/164622/bubble-and-squeak".to_string(),
                str_meal_thumb: "https://www.themealdb.com/images/media/meals/xusqvw1511638311.jpg".to_string(),
                str_ingredient1: Some("ingredient 1".to_string()),
                str_ingredient2: Some("ingredient 2".to_string()),
                str_ingredient3: Some("ingredient 3".to_string()),
                str_ingredient4: Some("ingredient 4".to_string()),
                str_ingredient5: Some("ingredient 5".to_string()),
                str_measure1: None,
                str_measure2: None,
                str_measure3: Some("400 g".to_string()),
                str_measure4: Some("400 g".to_string()),
                str_measure5: Some("400 g".to_string()),
            }]
        }
    }
}


pub async fn fetch(url: String, meth: String) -> Result<Recipe, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&meth);
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept","application/json");

    let win = web_sys::window().expect("Couldn't get window object");
    let resp_value =  JsFuture::from(win.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    info!("json: {:?}", &json);
    let branch_info: Recipe= json.into_serde().expect("Couldn't serialize into T");
    info!("obj: {:?}", &branch_info);
    //Ok(JsValue::from_serde(&branch_info).unwrap())
    Ok(branch_info)
}