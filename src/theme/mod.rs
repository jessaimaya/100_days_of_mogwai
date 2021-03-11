use css_colors::*;
use lazy_static::*;

#[derive(Copy, Clone, Debug)]
pub struct ThemeApp {
    pub background: ColTheme,
}

#[derive(Copy, Clone, Debug)]
pub struct ColTheme {
    pub light: RGB,
    pub dark: RGB,
}

impl std::default::Default for ThemeApp {
    fn default() -> Self {
        ThemeApp {
            background: ColTheme{ light: rgb(16, 17, 20), dark: rgb(16, 17, 20)},
        }
    }
}

lazy_static! {
    pub static ref THEME:ThemeApp = ThemeApp::default();
}