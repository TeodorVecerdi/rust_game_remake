use conrod_core::color::Color;
use lazy_static::lazy_static;

pub const DARK_THEME: Theme = Theme::const_default();

pub const LIGHT_THEME: Theme = Theme {
    primary_text: OXFORD_BLUE,
    secondary_text: MINT_CREAM,
    background: MINT_CREAM,
    panel_dark: INDEPENDANCE,
    panel_light: RAISIN_BLACK,
    accent_color: IMPERIAL_RED,
    accend_color_secondary: PACIFIC_BLUE,

    button_normal: rgbi(0xafafaf),
    button_hover: rgbi(0x888888),
    button_pressed: rgbi(0x9f9f9f),
    button_disabled: rgbi(0x555555),
};

pub const MINT_CREAM: Color = rgb(246, 255, 255);
pub const IMPERIAL_RED: Color = rgb(233, 54, 64);
pub const PACIFIC_BLUE: Color = rgb(15, 163, 184);
pub const OXFORD_BLUE: Color = rgb(0, 0, 25);
pub const INDEPENDANCE: Color = rgb(73, 71, 91);
pub const RAISIN_BLACK: Color = rgb(32, 32, 48);

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary_text: Color,
    pub secondary_text: Color,
    pub background: Color,
    pub panel_dark: Color,
    pub panel_light: Color,
    pub accent_color: Color,
    pub accend_color_secondary: Color,

    pub button_normal: Color,
    pub button_hover: Color,
    pub button_pressed: Color,
    pub button_disabled: Color,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ThemeOption {
    PrimaryText(Color),
    SecondaryText(Color),
    Background(Color),
    PanelDark(Color),
    PanelLight(Color),
    AccentColor(Color),
    AccendColorSecondary(Color),
    ButtonNormal(Color),
    ButtonHover(Color),
    ButtonPressed(Color),
    ButtonDisabled(Color),
}

#[allow(dead_code)]
impl Theme {
    pub const fn const_default() -> Self {
        Theme {
            primary_text: MINT_CREAM,
            secondary_text: OXFORD_BLUE,
            background: OXFORD_BLUE,
            panel_dark: RAISIN_BLACK,
            panel_light: INDEPENDANCE,
            accent_color: IMPERIAL_RED,
            accend_color_secondary: PACIFIC_BLUE,

            button_normal: rgbi(0xffffff),
            button_hover: rgb(200, 200, 200),
            button_pressed: rgb(220, 220, 220),
            button_disabled: rgbi(0x555555),
        }
    }

    pub fn with_primary_text(mut self, color: Color) -> Self {
        self.primary_text = color;
        self
    }

    pub fn with_secondary_text(mut self, color: Color) -> Self {
        self.secondary_text = color;
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn with_panel_dark(mut self, color: Color) -> Self {
        self.panel_dark = color;
        self
    }

    pub fn with_panel_light(mut self, color: Color) -> Self {
        self.panel_light = color;
        self
    }

    pub fn with_accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }

    pub fn with_accent_color_secondary(mut self, color: Color) -> Self {
        self.accend_color_secondary = color;
        self
    }

    pub fn with_button_normal(mut self, color: Color) -> Self {
        self.button_normal = color;
        self
    }

    pub fn with_button_hover(mut self, color: Color) -> Self {
        self.button_hover = color;
        self
    }

    pub fn with_button_pressed(mut self, color: Color) -> Self {
        self.button_pressed = color;
        self
    }

    pub fn with_button_disabled(mut self, color: Color) -> Self {
        self.button_disabled = color;
        self
    }

    pub fn with_options(mut self, options: Vec<ThemeOption>) -> Self {
        for option in options {
            match option {
                ThemeOption::PrimaryText(color) => {
                    self.primary_text = color;
                },
                ThemeOption::SecondaryText(color) => {
                    self.secondary_text = color;
                },
                ThemeOption::Background(color) => {
                    self.background = color;
                },
                ThemeOption::PanelDark(color) => {
                    self.panel_dark = color;
                },
                ThemeOption::PanelLight(color) => {
                    self.panel_light = color;
                },
                ThemeOption::AccentColor(color) => {
                    self.accent_color = color;
                },
                ThemeOption::AccendColorSecondary(color) => {
                    self.accend_color_secondary = color;
                },
                ThemeOption::ButtonNormal(color) => {
                    self.button_normal = color;
                },
                ThemeOption::ButtonHover(color) => {
                    self.button_hover = color;
                },
                ThemeOption::ButtonPressed(color) => {
                    self.button_pressed = color;
                },
                ThemeOption::ButtonDisabled(color) => {
                    self.button_disabled = color;
                },
            }
        }
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary_text: MINT_CREAM,
            secondary_text: OXFORD_BLUE,
            background: OXFORD_BLUE,
            panel_dark: RAISIN_BLACK,
            panel_light: INDEPENDANCE,
            accent_color: IMPERIAL_RED,
            accend_color_secondary: PACIFIC_BLUE,

            button_normal: rgbi(0xffffff),
            button_hover: rgb(200, 200, 200),
            button_pressed: rgb(220, 220, 220),
            button_disabled: rgb(150, 150, 150),
        }
    }
}


pub struct ThemeManager {
    pub active_theme: Box<Theme>
}

impl ThemeManager {
    pub fn new() -> Self {
        ThemeManager {
            active_theme: Box::new(DARK_THEME)
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.active_theme = Box::new(theme);
    }
}


macro_rules! make_color {
	($r:expr, $g:expr, $b:expr) => ( Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0));
	($r:expr, $g:expr, $b:expr, $a:expr) => ( Color::Rgba($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, $a as f32 / 255.0));
}

// alias for color highlighter
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    make_color!(r, g, b)
}

const fn rgbi(color: u32) -> Color {
    make_color!((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF)
}
