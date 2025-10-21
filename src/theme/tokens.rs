use gpui::*;
use std::time::Duration;

use crate::fonts::{UI_FONT_FAMILY, UI_MONO_FONT_FAMILY};

/// Shadcn-inspired semantic color and layout tokens
#[derive(Clone, Debug)]
pub struct ThemeTokens {
    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,

    pub radius_sm: Pixels,
    pub radius_md: Pixels,
    pub radius_lg: Pixels,
    pub radius_xl: Pixels,

    pub shadow_xs: BoxShadow,
    pub shadow_sm: BoxShadow,
    pub shadow_md: BoxShadow,
    pub shadow_lg: BoxShadow,
    pub shadow_xl: BoxShadow,

    pub ring_offset: Pixels,

    pub transition_fast: Duration,
    pub transition_base: Duration,
    pub transition_slow: Duration,

    pub font_family: SharedString,
    pub font_mono: SharedString,
}

impl ThemeTokens {
    pub fn light() -> Self {
        Self {
            background: rgb(0xffffff).into(),
            foreground: rgb(0x0a0a0a).into(),
            card: rgb(0xffffff).into(),
            card_foreground: rgb(0x0a0a0a).into(),
            popover: rgb(0xffffff).into(),
            popover_foreground: rgb(0x0a0a0a).into(),
            muted: rgb(0xf5f5f5).into(),
            muted_foreground: rgb(0x737373).into(),
            accent: rgb(0xf5f5f5).into(),
            accent_foreground: rgb(0x0a0a0a).into(),
            primary: rgb(0x000000).into(),
            primary_foreground: rgb(0xffffff).into(),
            secondary: rgb(0xf5f5f5).into(),
            secondary_foreground: rgb(0x0a0a0a).into(),
            destructive: rgb(0xef4444).into(),
            destructive_foreground: rgb(0xffffff).into(),
            border: rgb(0xe5e5e5).into(),
            input: rgb(0xe5e5e5).into(),
            ring: rgb(0xd4d4d8).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.05),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: rgb(0x000000).into(),
            foreground: rgb(0xf5f5f5).into(),
            card: rgb(0x0a0a0a).into(),
            card_foreground: rgb(0xf5f5f5).into(),
            popover: rgb(0x0a0a0a).into(),
            popover_foreground: rgb(0xf5f5f5).into(),
            muted: rgb(0x1a1a1a).into(),
            muted_foreground: rgb(0x737373).into(),
            accent: rgb(0x262626).into(),
            accent_foreground: rgb(0xffffff).into(),
            primary: rgb(0xffffff).into(),
            primary_foreground: rgb(0x000000).into(),
            secondary: rgb(0x262626).into(),
            secondary_foreground: rgb(0xf5f5f5).into(),
            destructive: rgb(0xff4444).into(),
            destructive_foreground: rgb(0xffffff).into(),
            border: rgb(0x333333).into(),
            input: rgb(0x333333).into(),
            ring: rgb(0xffffff).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.5),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.6),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.7),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.8),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.9),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn midnight_blue() -> Self {
        Self {
            background: rgb(0x0a0f14).into(), // Darker for better contrast
            foreground: rgb(0xe6edf3).into(),
            card: rgb(0x0f1419).into(),
            card_foreground: rgb(0xe6edf3).into(),
            popover: rgb(0x0f1419).into(),
            popover_foreground: rgb(0xe6edf3).into(),
            muted: rgb(0x161b22).into(),
            muted_foreground: rgb(0x6e7681).into(),
            accent: rgb(0x1e3a8a).into(),
            accent_foreground: rgb(0x60a5fa).into(),
            primary: rgb(0x60a5fa).into(), // Brighter blue for better visibility
            primary_foreground: rgb(0x0a0f14).into(),
            secondary: rgb(0x1e293b).into(),
            secondary_foreground: rgb(0xe2e8f0).into(),
            destructive: rgb(0xef4444).into(),
            destructive_foreground: rgb(0xffffff).into(),
            border: rgb(0x21262d).into(),
            input: rgb(0x21262d).into(),
            ring: rgb(0x60a5fa).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn forest_grove() -> Self {
        Self {
            background: rgb(0x0a0e0b).into(), // Darker for contrast
            foreground: rgb(0xd4e5d4).into(),
            card: rgb(0x0f1410).into(),
            card_foreground: rgb(0xd4e5d4).into(),
            popover: rgb(0x0f1410).into(),
            popover_foreground: rgb(0xd4e5d4).into(),
            muted: rgb(0x141a15).into(),
            muted_foreground: rgb(0x6b7a6b).into(),
            accent: rgb(0x14532d).into(),
            accent_foreground: rgb(0x4ade80).into(),
            primary: rgb(0x4ade80).into(), // Brighter green for pop
            primary_foreground: rgb(0x0a0e0b).into(),
            secondary: rgb(0x1e3a26).into(),
            secondary_foreground: rgb(0xbbf7d0).into(),
            destructive: rgb(0xf87171).into(),
            destructive_foreground: rgb(0xffffff).into(),
            border: rgb(0x1a2520).into(),
            input: rgb(0x1a2520).into(),
            ring: rgb(0x4ade80).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn sunset_amber() -> Self {
        Self {
            background: rgb(0x140a05).into(), // Much darker for contrast
            foreground: rgb(0xfef3c7).into(),
            card: rgb(0x1c0f08).into(),
            card_foreground: rgb(0xfef3c7).into(),
            popover: rgb(0x1c0f08).into(),
            popover_foreground: rgb(0xfef3c7).into(),
            muted: rgb(0x2a1810).into(),
            muted_foreground: rgb(0x9d7c5a).into(),
            accent: rgb(0x7c2d12).into(),
            accent_foreground: rgb(0xfbbf24).into(),
            primary: rgb(0xfbbf24).into(), // Brighter amber for better pop
            primary_foreground: rgb(0x140a05).into(),
            secondary: rgb(0x4c1d95).into(),
            secondary_foreground: rgb(0xe9d5ff).into(),
            destructive: rgb(0xef4444).into(),
            destructive_foreground: rgb(0xffffff).into(),
            border: rgb(0x3a2415).into(),
            input: rgb(0x3a2415).into(),
            ring: rgb(0xfbbf24).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn ocean_breeze() -> Self {
        Self {
            background: rgb(0x051018).into(), // Darker ocean depth
            foreground: rgb(0xe3f2fd).into(),
            card: rgb(0x0a1929).into(),
            card_foreground: rgb(0xe3f2fd).into(),
            popover: rgb(0x0a1929).into(),
            popover_foreground: rgb(0xe3f2fd).into(),
            muted: rgb(0x0f2638).into(),
            muted_foreground: rgb(0x67a3b8).into(),
            accent: rgb(0x0e7490).into(),
            accent_foreground: rgb(0x22d3ee).into(),
            primary: rgb(0x22d3ee).into(), // Brighter cyan for visibility
            primary_foreground: rgb(0x051018).into(),
            secondary: rgb(0x0284c7).into(),
            secondary_foreground: rgb(0xe0f2fe).into(),
            destructive: rgb(0xf87171).into(),
            destructive_foreground: rgb(0xffffff).into(),
            border: rgb(0x0f2638).into(),
            input: rgb(0x0f2638).into(),
            ring: rgb(0x22d3ee).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn dracula() -> Self {
        Self {
            background: rgb(0x1e1f29).into(), // Darker Dracula
            foreground: rgb(0xf8f8f2).into(),
            card: rgb(0x282a36).into(),
            card_foreground: rgb(0xf8f8f2).into(),
            popover: rgb(0x282a36).into(),
            popover_foreground: rgb(0xf8f8f2).into(),
            muted: rgb(0x44475a).into(),
            muted_foreground: rgb(0x6272a4).into(),
            accent: rgb(0x44475a).into(),
            accent_foreground: rgb(0xf8f8f2).into(),
            primary: rgb(0xc9a9ff).into(), // Brighter purple for more pop
            primary_foreground: rgb(0x1e1f29).into(),
            secondary: rgb(0x44475a).into(),
            secondary_foreground: rgb(0xf8f8f2).into(),
            destructive: rgb(0xff6e6e).into(),
            destructive_foreground: rgb(0xf8f8f2).into(),
            border: rgb(0x3a3c4e).into(),
            input: rgb(0x3a3c4e).into(),
            ring: rgb(0xc9a9ff).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn nord() -> Self {
        Self {
            background: rgb(0x242933).into(), // Darker Nord
            foreground: rgb(0xeceff4).into(),
            card: rgb(0x2e3440).into(),
            card_foreground: rgb(0xeceff4).into(),
            popover: rgb(0x2e3440).into(),
            popover_foreground: rgb(0xeceff4).into(),
            muted: rgb(0x3b4252).into(),
            muted_foreground: rgb(0x81a1c1).into(),
            accent: rgb(0x434c5e).into(),
            accent_foreground: rgb(0x8fbcbb).into(),
            primary: rgb(0x8fbcbb).into(), // Brighter frost cyan
            primary_foreground: rgb(0x242933).into(),
            secondary: rgb(0x5e81ac).into(),
            secondary_foreground: rgb(0xeceff4).into(),
            destructive: rgb(0xbf616a).into(),
            destructive_foreground: rgb(0xeceff4).into(),
            border: rgb(0x3b4252).into(),
            input: rgb(0x3b4252).into(),
            ring: rgb(0x8fbcbb).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn monokai_pro() -> Self {
        Self {
            background: rgb(0x221f22).into(), // Darker Monokai
            foreground: rgb(0xfcfcfa).into(),
            card: rgb(0x2d2a2e).into(),
            card_foreground: rgb(0xfcfcfa).into(),
            popover: rgb(0x2d2a2e).into(),
            popover_foreground: rgb(0xfcfcfa).into(),
            muted: rgb(0x403e41).into(),
            muted_foreground: rgb(0x939293).into(),
            accent: rgb(0x5b595c).into(),
            accent_foreground: rgb(0xfcfcfa).into(),
            primary: rgb(0xffe66d).into(), // Brighter yellow for more pop
            primary_foreground: rgb(0x221f22).into(),
            secondary: rgb(0x5b595c).into(),
            secondary_foreground: rgb(0xfcfcfa).into(),
            destructive: rgb(0xff6e97).into(),
            destructive_foreground: rgb(0xfcfcfa).into(),
            border: rgb(0x403e41).into(),
            input: rgb(0x403e41).into(),
            ring: rgb(0xffe66d).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            background: rgb(0x16161e).into(), // Darker Tokyo Night
            foreground: rgb(0xc0caf5).into(),
            card: rgb(0x1a1b26).into(),
            card_foreground: rgb(0xc0caf5).into(),
            popover: rgb(0x1a1b26).into(),
            popover_foreground: rgb(0xc0caf5).into(),
            muted: rgb(0x1f2335).into(),
            muted_foreground: rgb(0x565f89).into(),
            accent: rgb(0x292e42).into(),
            accent_foreground: rgb(0x7aa2f7).into(),
            primary: rgb(0x7dcfff).into(), // Brighter blue for Tokyo Night
            primary_foreground: rgb(0x16161e).into(),
            secondary: rgb(0x292e42).into(),
            secondary_foreground: rgb(0xc0caf5).into(),
            destructive: rgb(0xf7768e).into(),
            destructive_foreground: rgb(0xc0caf5).into(),
            border: rgb(0x292e42).into(),
            input: rgb(0x292e42).into(),
            ring: rgb(0x7dcfff).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            background: rgb(0x181825).into(), // Darker Catppuccin
            foreground: rgb(0xcdd6f4).into(),
            card: rgb(0x1e1e2e).into(),
            card_foreground: rgb(0xcdd6f4).into(),
            popover: rgb(0x1e1e2e).into(),
            popover_foreground: rgb(0xcdd6f4).into(),
            muted: rgb(0x313244).into(),
            muted_foreground: rgb(0x7f849c).into(),
            accent: rgb(0x45475a).into(),
            accent_foreground: rgb(0x89b4fa).into(),
            primary: rgb(0x89b4fa).into(), // Brighter lavender blue
            primary_foreground: rgb(0x181825).into(),
            secondary: rgb(0x585b70).into(),
            secondary_foreground: rgb(0xcdd6f4).into(),
            destructive: rgb(0xf38ba8).into(),
            destructive_foreground: rgb(0xcdd6f4).into(),
            border: rgb(0x313244).into(),
            input: rgb(0x313244).into(),
            ring: rgb(0x89b4fa).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn rose_pine() -> Self {
        Self {
            background: rgb(0x131019).into(), // Darker Rose Pine
            foreground: rgb(0xe0def4).into(),
            card: rgb(0x191724).into(),
            card_foreground: rgb(0xe0def4).into(),
            popover: rgb(0x191724).into(),
            popover_foreground: rgb(0xe0def4).into(),
            muted: rgb(0x1f1d2e).into(),
            muted_foreground: rgb(0x6e6a86).into(),
            accent: rgb(0x26233a).into(),
            accent_foreground: rgb(0xc4a7e7).into(),
            primary: rgb(0xc4a7e7).into(), // Beautiful iris purple
            primary_foreground: rgb(0x131019).into(),
            secondary: rgb(0x2a273f).into(),
            secondary_foreground: rgb(0xe0def4).into(),
            destructive: rgb(0xeb6f92).into(),
            destructive_foreground: rgb(0xe0def4).into(),
            border: rgb(0x26233a).into(),
            input: rgb(0x26233a).into(),
            ring: rgb(0xc4a7e7).into(),

            radius_sm: px(4.0),
            radius_md: px(6.0),
            radius_lg: px(8.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn coral_reef() -> Self {
        Self {
            background: rgb(0xFFFBF8).into(), // Soft cream background
            foreground: rgb(0x2D3748).into(), // Dark slate text
            card: rgb(0xFFFFFF).into(), // Pure white cards
            card_foreground: rgb(0x2D3748).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x2D3748).into(),
            muted: rgb(0xFFF5F0).into(), // Light coral tint
            muted_foreground: rgb(0x718096).into(), // Medium gray
            accent: rgb(0xFFE4D6).into(), // Peach accent
            accent_foreground: rgb(0x2D3748).into(),
            primary: rgb(0xFF6B6B).into(), // Vibrant coral
            primary_foreground: rgb(0xFFFFFF).into(),
            secondary: rgb(0x4FD1C5).into(), // Turquoise
            secondary_foreground: rgb(0xFFFFFF).into(),
            destructive: rgb(0xF56565).into(), // Bright red
            destructive_foreground: rgb(0xFFFFFF).into(),
            border: rgb(0xFFD4C2).into(), // Soft coral border
            input: rgb(0xFFE4D6).into(),
            ring: rgb(0xFF6B6B).into(), // Coral ring

            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn lavender_dreams() -> Self {
        Self {
            background: rgb(0xF8F7FF).into(), // Soft lavender white
            foreground: rgb(0x2D2A3D).into(), // Deep purple-gray
            card: rgb(0xFFFFFF).into(), // Pure white
            card_foreground: rgb(0x2D2A3D).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x2D2A3D).into(),
            muted: rgb(0xF0EDFF).into(), // Light lavender
            muted_foreground: rgb(0x6B6880).into(), // Purple-gray
            accent: rgb(0xE8E3FF).into(), // Soft purple
            accent_foreground: rgb(0x2D2A3D).into(),
            primary: rgb(0x9F7AEA).into(), // Vibrant lavender
            primary_foreground: rgb(0xFFFFFF).into(),
            secondary: rgb(0xB794F6).into(), // Light purple
            secondary_foreground: rgb(0xFFFFFF).into(),
            destructive: rgb(0xE53E3E).into(), // Red
            destructive_foreground: rgb(0xFFFFFF).into(),
            border: rgb(0xD6CEFF).into(), // Lavender border
            input: rgb(0xE8E3FF).into(),
            ring: rgb(0x9F7AEA).into(), // Lavender ring

            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn mint_fresh() -> Self {
        Self {
            background: rgb(0xF7FDFB).into(), // Mint-tinted white
            foreground: rgb(0x1A4D3C).into(), // Forest green text
            card: rgb(0xFFFFFF).into(), // Pure white
            card_foreground: rgb(0x1A4D3C).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x1A4D3C).into(),
            muted: rgb(0xE6F9F3).into(), // Soft mint
            muted_foreground: rgb(0x4A7C69).into(), // Medium green
            accent: rgb(0xD4F4E8).into(), // Light mint
            accent_foreground: rgb(0x1A4D3C).into(),
            primary: rgb(0x38B2AC).into(), // Teal/turquoise
            primary_foreground: rgb(0xFFFFFF).into(),
            secondary: rgb(0x48BB78).into(), // Fresh green
            secondary_foreground: rgb(0xFFFFFF).into(),
            destructive: rgb(0xF56565).into(), // Red
            destructive_foreground: rgb(0xFFFFFF).into(),
            border: rgb(0xB8EBD9).into(), // Mint border
            input: rgb(0xD4F4E8).into(),
            ring: rgb(0x38B2AC).into(), // Teal ring

            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn peachy_keen() -> Self {
        Self {
            background: rgb(0xFFFAF5).into(), // Warm cream
            foreground: rgb(0x3D2817).into(), // Dark brown
            card: rgb(0xFFFFFF).into(), // Pure white
            card_foreground: rgb(0x3D2817).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x3D2817).into(),
            muted: rgb(0xFFF0E0).into(), // Soft peach
            muted_foreground: rgb(0x8B6B47).into(), // Warm brown
            accent: rgb(0xFFE4CC).into(), // Light peach
            accent_foreground: rgb(0x3D2817).into(),
            primary: rgb(0xFF9966).into(), // Vibrant peach
            primary_foreground: rgb(0xFFFFFF).into(),
            secondary: rgb(0xFFB84D).into(), // Warm orange
            secondary_foreground: rgb(0xFFFFFF).into(),
            destructive: rgb(0xE53E3E).into(), // Red
            destructive_foreground: rgb(0xFFFFFF).into(),
            border: rgb(0xFFD9B3).into(), // Peach border
            input: rgb(0xFFE4CC).into(),
            ring: rgb(0xFF9966).into(), // Peach ring

            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn sky_blue() -> Self {
        Self {
            background: rgb(0xF7FAFD).into(), // Sky-tinted white
            foreground: rgb(0x1E3A5F).into(), // Deep blue text
            card: rgb(0xFFFFFF).into(), // Pure white
            card_foreground: rgb(0x1E3A5F).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x1E3A5F).into(),
            muted: rgb(0xE3F2FD).into(), // Light blue
            muted_foreground: rgb(0x5B7C99).into(), // Steel blue
            accent: rgb(0xBBDEFB).into(), // Sky blue accent
            accent_foreground: rgb(0x1E3A5F).into(),
            primary: rgb(0x2196F3).into(), // Bright blue
            primary_foreground: rgb(0xFFFFFF).into(),
            secondary: rgb(0x42A5F5).into(), // Light bright blue
            secondary_foreground: rgb(0xFFFFFF).into(),
            destructive: rgb(0xF44336).into(), // Red
            destructive_foreground: rgb(0xFFFFFF).into(),
            border: rgb(0x90CAF9).into(), // Sky blue border
            input: rgb(0xBBDEFB).into(),
            ring: rgb(0x2196F3).into(), // Blue ring

            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }

    pub fn cherry_blossom() -> Self {
        Self {
            background: rgb(0xFFF8FB).into(), // Soft pink-white
            foreground: rgb(0x4A1942).into(), // Deep magenta text
            card: rgb(0xFFFFFF).into(), // Pure white
            card_foreground: rgb(0x4A1942).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x4A1942).into(),
            muted: rgb(0xFFE8F5).into(), // Light pink
            muted_foreground: rgb(0x9B4F96).into(), // Mauve
            accent: rgb(0xFFD6ED).into(), // Soft pink accent
            accent_foreground: rgb(0x4A1942).into(),
            primary: rgb(0xE91E63).into(), // Vibrant pink/magenta
            primary_foreground: rgb(0xFFFFFF).into(),
            secondary: rgb(0xF06292).into(), // Cherry pink
            secondary_foreground: rgb(0xFFFFFF).into(),
            destructive: rgb(0xE53935).into(), // Red
            destructive_foreground: rgb(0xFFFFFF).into(),
            border: rgb(0xFFB3D9).into(), // Pink border
            input: rgb(0xFFD6ED).into(),
            ring: rgb(0xE91E63).into(), // Pink ring

            radius_sm: px(6.0),
            radius_md: px(8.0),
            radius_lg: px(12.0),
            radius_xl: px(12.0),

            shadow_xs: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),
        }
    }
}

impl ThemeTokens {
    /// Create a focus ring shadow (3px spread with opacity)
    pub fn focus_ring(&self, opacity: f32) -> BoxShadow {
        BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            color: self.ring.opacity(opacity),
        }
    }

    /// Create a focus ring for light backgrounds
    pub fn focus_ring_light(&self) -> BoxShadow {
        self.focus_ring(0.5)
    }

    /// Create a focus ring for dark backgrounds
    pub fn focus_ring_dark(&self) -> BoxShadow {
        self.focus_ring(0.4)
    }

    /// Create a validation error ring
    pub fn error_ring(&self) -> BoxShadow {
        BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            color: self.destructive.opacity(0.2),
        }
    }

    /// Create a success ring (for validated inputs)
    pub fn success_ring(&self) -> BoxShadow {
        let success_color = hsla(0.33, 0.70, 0.50, 1.0); // Green
        BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            color: success_color.opacity(0.2),
        }
    }
}


