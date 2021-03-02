use css_colors::{Color, RGB, rgb};

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub background: ColTheme,
    pub foreground: ColTheme,
    pub accent: ColTheme,
}

#[derive(Copy, Clone, Debug)]
pub struct ColTheme {
    pub light: RGB,
    pub dark: RGB,
}

impl std::default::Default for Theme {
    fn default() -> Self {
        Theme {
            background: ColTheme{ light: rgb(148, 176, 138), dark: rgb(19, 38, 40)},
            foreground: ColTheme{ light: rgb(19, 38, 40), dark: rgb(148, 176, 138)},
            accent: ColTheme{ light: rgb(181, 93, 90), dark: rgb(145, 53, 79)},
        }
    }
}