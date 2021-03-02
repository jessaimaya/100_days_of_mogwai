use css_colors::{RGB, rgb, Color};
use lazy_static::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct ThemeApp {
    pub background: ColTheme,
    pub foreground: ColTheme,
    pub accent: ColTheme,
}

#[derive(Copy, Clone, Debug)]
pub struct ColTheme {
    pub light: RGB,
    pub dark: RGB,
}

impl std::default::Default for ThemeApp {
    fn default() -> Self {
        ThemeApp {
            background: ColTheme{ light: rgb(148, 176, 138), dark: rgb(19, 38, 40)},
            foreground: ColTheme{ light: rgb(19, 38, 40), dark: rgb(148, 176, 138)},
            accent: ColTheme{ light: rgb(181, 93, 90), dark: rgb(145, 53, 79)},
        }
    }
}

lazy_static! {
    pub static ref Theme:ThemeApp = ThemeApp::default();
}