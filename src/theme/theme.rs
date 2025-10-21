use gpui::*;
use once_cell::sync::Lazy;

use super::tokens::ThemeTokens;

/// Theme variants
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Light theme
    Light,
    /// Dark theme
    Dark,
    /// Midnight Blue - Deep, calming dark blue tones
    MidnightBlue,
    /// Forest Grove - Natural greens with earthy accents
    ForestGrove,
    /// Sunset Amber - Warm oranges and deep purples
    SunsetAmber,
    /// Ocean Breeze - Cool blues and teals
    OceanBreeze,
    /// Dracula - Popular purple-based dark theme
    Dracula,
    /// Nord - Arctic, bluish color palette
    Nord,
    /// Monokai Pro - Vibrant syntax highlighting colors
    MonokaiPro,
    /// Tokyo Night - Modern dark theme with purple accents
    TokyoNight,
    /// Catppuccin Mocha - Pastel dark theme
    CatppuccinMocha,
    /// Rose Pine - Muted, natural tones
    RosePine,
    /// Coral Reef - Vibrant coral and turquoise
    CoralReef,
    /// Lavender Dreams - Soft purples and pastels
    LavenderDreams,
    /// Mint Fresh - Cool mint greens with clean whites
    MintFresh,
    /// Peachy Keen - Warm peach and orange tones
    PeachyKeen,
    /// Sky Blue - Bright blues inspired by clear skies
    SkyBlue,
    /// Cherry Blossom - Pink and magenta spring colors
    CherryBlossom,
}

/// GPUI-accessible theme wrapper
#[derive(Clone, Debug)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub tokens: ThemeTokens,
}

impl Theme {
    pub fn light() -> Self {
        Self { variant: ThemeVariant::Light, tokens: ThemeTokens::light() }
    }
    pub fn dark() -> Self {
        Self { variant: ThemeVariant::Dark, tokens: ThemeTokens::dark() }
    }
    pub fn midnight_blue() -> Self {
        Self { variant: ThemeVariant::MidnightBlue, tokens: ThemeTokens::midnight_blue() }
    }
    pub fn forest_grove() -> Self {
        Self { variant: ThemeVariant::ForestGrove, tokens: ThemeTokens::forest_grove() }
    }
    pub fn sunset_amber() -> Self {
        Self { variant: ThemeVariant::SunsetAmber, tokens: ThemeTokens::sunset_amber() }
    }
    pub fn ocean_breeze() -> Self {
        Self { variant: ThemeVariant::OceanBreeze, tokens: ThemeTokens::ocean_breeze() }
    }
    pub fn dracula() -> Self {
        Self { variant: ThemeVariant::Dracula, tokens: ThemeTokens::dracula() }
    }
    pub fn nord() -> Self {
        Self { variant: ThemeVariant::Nord, tokens: ThemeTokens::nord() }
    }
    pub fn monokai_pro() -> Self {
        Self { variant: ThemeVariant::MonokaiPro, tokens: ThemeTokens::monokai_pro() }
    }
    pub fn tokyo_night() -> Self {
        Self { variant: ThemeVariant::TokyoNight, tokens: ThemeTokens::tokyo_night() }
    }
    pub fn catppuccin_mocha() -> Self {
        Self { variant: ThemeVariant::CatppuccinMocha, tokens: ThemeTokens::catppuccin_mocha() }
    }
    pub fn rose_pine() -> Self {
        Self { variant: ThemeVariant::RosePine, tokens: ThemeTokens::rose_pine() }
    }
    pub fn coral_reef() -> Self {
        Self { variant: ThemeVariant::CoralReef, tokens: ThemeTokens::coral_reef() }
    }
    pub fn lavender_dreams() -> Self {
        Self { variant: ThemeVariant::LavenderDreams, tokens: ThemeTokens::lavender_dreams() }
    }
    pub fn mint_fresh() -> Self {
        Self { variant: ThemeVariant::MintFresh, tokens: ThemeTokens::mint_fresh() }
    }
    pub fn peachy_keen() -> Self {
        Self { variant: ThemeVariant::PeachyKeen, tokens: ThemeTokens::peachy_keen() }
    }
    pub fn sky_blue() -> Self {
        Self { variant: ThemeVariant::SkyBlue, tokens: ThemeTokens::sky_blue() }
    }
    pub fn cherry_blossom() -> Self {
        Self { variant: ThemeVariant::CherryBlossom, tokens: ThemeTokens::cherry_blossom() }
    }
}

static THEME_STATE: Lazy<std::sync::Mutex<Theme>> = Lazy::new(|| std::sync::Mutex::new(Theme::dark()));

/// Install a theme globally for the app. Call early during app startup.
pub fn install_theme(_cx: &mut App, theme: Theme) {
    if let Ok(mut state) = THEME_STATE.lock() {
        *state = theme;
    }
}

/// Access the current theme tokens.
pub fn use_theme() -> Theme {
    THEME_STATE.lock().map(|guard| (*guard).clone()).unwrap_or_else(|_| Theme::dark())
}


