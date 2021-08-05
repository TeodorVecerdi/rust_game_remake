use conrod_core::color::Color;
use lazy_static::lazy_static;

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

/* lazy_static! {
    // static ref DEFAULT_THEME: Theme = Theme::default();
    pub static ref ACTIVE_THEME: Box<Theme> = Box::new(DARK_THEME);
} */

const MINT_CREAM: Color = rgb(246, 255, 255);
const IMPERIAL_RED: Color = rgb(233, 54, 64);
const PACIFIC_BLUE: Color = rgb(15, 163, 184);
const OXFORD_BLUE: Color = rgb(0, 0, 25);
const INDEPENDANCE: Color = rgb(73, 71, 91);
const RAISIN_BLACK: Color = rgb(32, 32, 48);

impl Theme {
    const fn const_default() -> Self {
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
