use conrod_core::color::Color;

pub const DARK_THEME: Theme = Theme::const_default();

pub const LIGHT_THEME: Theme = Theme {
    text_primary: OXFORD_BLUE,
    text_secondary: MINT_CREAM,
    text_light: MINT_CREAM,
    text_dark: OXFORD_BLUE,

    background: MINT_CREAM,
    panel_dark: INDEPENDANCE,
    panel_light: RAISIN_BLACK,
    accent_color: IMPERIAL_RED,
    accend_color_secondary: PACIFIC_BLUE,

    button_normal: rgbi(0xafafaf),
    button_hover: rgbi(0x888888),
    button_press: rgbi(0x666666),
    button_disabled: rgbi(0x555555),
};

pub const MINT_CREAM: Color = rgb(246, 255, 255);
pub const IMPERIAL_RED: Color = rgb(233, 54, 64);
pub const PACIFIC_BLUE: Color = rgb(15, 163, 184);
pub const OXFORD_BLUE: Color = rgb(0, 0, 25);
pub const INDEPENDANCE: Color = rgbi(0xdadada);
pub const RAISIN_BLACK: Color = rgb(32, 32, 48);
pub const TRANSPARENT: Color = rgbai(0x00000000);

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_light: Color,
    pub text_dark: Color,

    pub background: Color,
    pub panel_dark: Color,
    pub panel_light: Color,
    pub accent_color: Color,
    pub accend_color_secondary: Color,

    pub button_normal: Color,
    pub button_hover: Color,
    pub button_press: Color,
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
            text_primary: MINT_CREAM,
            text_secondary: OXFORD_BLUE,
            text_light: MINT_CREAM,
            text_dark: OXFORD_BLUE,

            background: OXFORD_BLUE,
            panel_dark: RAISIN_BLACK,
            panel_light: INDEPENDANCE,
            accent_color: IMPERIAL_RED,
            accend_color_secondary: PACIFIC_BLUE,

            button_normal: rgbi(0xdadada),
            button_hover: rgbi(0xafafaf),
            button_press: rgbi(0xffffff),
            button_disabled: rgbi(0x555555),
        }
    }

    pub const fn with_primary_text(mut self, color: Color) -> Self {
        self.text_primary = color;
        self
    }

    pub const fn with_secondary_text(mut self, color: Color) -> Self {
        self.text_secondary = color;
        self
    }

    pub const fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub const fn with_panel_dark(mut self, color: Color) -> Self {
        self.panel_dark = color;
        self
    }

    pub const fn with_panel_light(mut self, color: Color) -> Self {
        self.panel_light = color;
        self
    }

    pub const fn with_accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }

    pub const fn with_accent_color_secondary(mut self, color: Color) -> Self {
        self.accend_color_secondary = color;
        self
    }

    pub const fn with_button_normal(mut self, color: Color) -> Self {
        self.button_normal = color;
        self
    }

    pub const fn with_button_hover(mut self, color: Color) -> Self {
        self.button_hover = color;
        self
    }

    pub const fn with_button_pressed(mut self, color: Color) -> Self {
        self.button_press = color;
        self
    }

    pub const fn with_button_disabled(mut self, color: Color) -> Self {
        self.button_disabled = color;
        self
    }

    pub const fn with_options(mut self, options: &[ThemeOption]) -> Self {
        let mut i = 0;
        while i < options.len() {
            match options[i] {
                ThemeOption::PrimaryText(color) => {
                    self.text_primary = color;
                },
                ThemeOption::SecondaryText(color) => {
                    self.text_secondary = color;
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
                    self.button_press = color;
                },
                ThemeOption::ButtonDisabled(color) => {
                    self.button_disabled = color;
                },
            }
            i+=1;
        }
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::const_default()
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
#[allow(dead_code)]
pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    make_color!(r, g, b)
}

#[allow(dead_code)]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    make_color!(r, g, b, a)
}

#[allow(dead_code)]
pub const fn rgbi(color: u32) -> Color {
    make_color!((color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF)
}

#[allow(dead_code)]
pub const fn rgbai(color: u32) -> Color {
    make_color!((color >> 24) & 0xFF, (color >> 16) & 0xFF, (color >> 8) & 0xFF, color & 0xFF)
}