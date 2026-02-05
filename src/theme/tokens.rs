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

    pub spacing_1: Pixels,
    pub spacing_2: Pixels,
    pub spacing_3: Pixels,
    pub spacing_4: Pixels,
    pub spacing_5: Pixels,
    pub spacing_6: Pixels,
    pub spacing_8: Pixels,
    pub spacing_10: Pixels,
    pub spacing_12: Pixels,
    pub spacing_16: Pixels,

    pub duration_fastest: Duration,
    pub duration_faster: Duration,
    pub duration_fast: Duration,
    pub duration_normal: Duration,
    pub duration_slow: Duration,
    pub duration_slower: Duration,
    pub duration_slowest: Duration,

    pub z_dropdown: u32,
    pub z_sticky: u32,
    pub z_modal: u32,
    pub z_popover: u32,
    pub z_tooltip: u32,
}

impl ThemeTokens {
    fn standard_spacing_duration_z() -> (
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Pixels,
        Duration,
        Duration,
        Duration,
        Duration,
        Duration,
        Duration,
        Duration,
        u32,
        u32,
        u32,
        u32,
        u32,
    ) {
        (
            px(4.0),
            px(8.0),
            px(12.0),
            px(16.0),
            px(20.0),
            px(24.0),
            px(32.0),
            px(40.0),
            px(48.0),
            px(64.0),
            Duration::from_millis(50),
            Duration::from_millis(100),
            Duration::from_millis(150),
            Duration::from_millis(200),
            Duration::from_millis(300),
            Duration::from_millis(400),
            Duration::from_millis(500),
            1000,
            1100,
            1300,
            1400,
            1500,
        )
    }

    fn apply_standard(mut self) -> Self {
        let (
            s1,
            s2,
            s3,
            s4,
            s5,
            s6,
            s8,
            s10,
            s12,
            s16,
            df,
            dfr,
            dfa,
            dn,
            ds,
            dsr,
            dst,
            zd,
            zs,
            zm,
            zp,
            zt,
        ) = Self::standard_spacing_duration_z();
        self.spacing_1 = s1;
        self.spacing_2 = s2;
        self.spacing_3 = s3;
        self.spacing_4 = s4;
        self.spacing_5 = s5;
        self.spacing_6 = s6;
        self.spacing_8 = s8;
        self.spacing_10 = s10;
        self.spacing_12 = s12;
        self.spacing_16 = s16;
        self.duration_fastest = df;
        self.duration_faster = dfr;
        self.duration_fast = dfa;
        self.duration_normal = dn;
        self.duration_slow = ds;
        self.duration_slower = dsr;
        self.duration_slowest = dst;
        self.z_dropdown = zd;
        self.z_sticky = zs;
        self.z_modal = zm;
        self.z_popover = zp;
        self.z_tooltip = zt;
        self
    }

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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.05),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.1),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.5),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.6),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.7),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.8),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.9),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }

    pub fn coral_reef() -> Self {
        Self {
            background: rgb(0xFFFBF8).into(), // Soft cream background
            foreground: rgb(0x2D3748).into(), // Dark slate text
            card: rgb(0xFFFFFF).into(),       // Pure white cards
            card_foreground: rgb(0x2D3748).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x2D3748).into(),
            muted: rgb(0xFFF5F0).into(),            // Light coral tint
            muted_foreground: rgb(0x718096).into(), // Medium gray
            accent: rgb(0xFFE4D6).into(),           // Peach accent
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }

    pub fn lavender_dreams() -> Self {
        Self {
            background: rgb(0xF8F7FF).into(), // Soft lavender white
            foreground: rgb(0x2D2A3D).into(), // Deep purple-gray
            card: rgb(0xFFFFFF).into(),       // Pure white
            card_foreground: rgb(0x2D2A3D).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x2D2A3D).into(),
            muted: rgb(0xF0EDFF).into(),            // Light lavender
            muted_foreground: rgb(0x6B6880).into(), // Purple-gray
            accent: rgb(0xE8E3FF).into(),           // Soft purple
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }

    pub fn mint_fresh() -> Self {
        Self {
            background: rgb(0xF7FDFB).into(), // Mint-tinted white
            foreground: rgb(0x1A4D3C).into(), // Forest green text
            card: rgb(0xFFFFFF).into(),       // Pure white
            card_foreground: rgb(0x1A4D3C).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x1A4D3C).into(),
            muted: rgb(0xE6F9F3).into(),            // Soft mint
            muted_foreground: rgb(0x4A7C69).into(), // Medium green
            accent: rgb(0xD4F4E8).into(),           // Light mint
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }

    pub fn peachy_keen() -> Self {
        Self {
            background: rgb(0xFFFAF5).into(), // Warm cream
            foreground: rgb(0x3D2817).into(), // Dark brown
            card: rgb(0xFFFFFF).into(),       // Pure white
            card_foreground: rgb(0x3D2817).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x3D2817).into(),
            muted: rgb(0xFFF0E0).into(),            // Soft peach
            muted_foreground: rgb(0x8B6B47).into(), // Warm brown
            accent: rgb(0xFFE4CC).into(),           // Light peach
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }

    pub fn sky_blue() -> Self {
        Self {
            background: rgb(0xF7FAFD).into(), // Sky-tinted white
            foreground: rgb(0x1E3A5F).into(), // Deep blue text
            card: rgb(0xFFFFFF).into(),       // Pure white
            card_foreground: rgb(0x1E3A5F).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x1E3A5F).into(),
            muted: rgb(0xE3F2FD).into(),            // Light blue
            muted_foreground: rgb(0x5B7C99).into(), // Steel blue
            accent: rgb(0xBBDEFB).into(),           // Sky blue accent
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }

    pub fn cherry_blossom() -> Self {
        Self {
            background: rgb(0xFFF8FB).into(), // Soft pink-white
            foreground: rgb(0x4A1942).into(), // Deep magenta text
            card: rgb(0xFFFFFF).into(),       // Pure white
            card_foreground: rgb(0x4A1942).into(),
            popover: rgb(0xFFFFFF).into(),
            popover_foreground: rgb(0x4A1942).into(),
            muted: rgb(0xFFE8F5).into(),            // Light pink
            muted_foreground: rgb(0x9B4F96).into(), // Mauve
            accent: rgb(0xFFD6ED).into(),           // Soft pink accent
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
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_sm: BoxShadow {
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.2),
            },
            shadow_md: BoxShadow {
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(6.0),
                spread_radius: px(-1.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_lg: BoxShadow {
                offset: point(px(0.0), px(10.0)),
                blur_radius: px(15.0),
                spread_radius: px(-3.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },
            shadow_xl: BoxShadow {
                offset: point(px(0.0), px(20.0)),
                blur_radius: px(25.0),
                spread_radius: px(-5.0),
                inset: false,
                color: hsla(0.0, 0.0, 0.0, 0.15),
            },

            ring_offset: px(2.0),

            transition_fast: Duration::from_millis(150),
            transition_base: Duration::from_millis(200),
            transition_slow: Duration::from_millis(300),

            font_family: UI_FONT_FAMILY.into(),
            font_mono: UI_MONO_FONT_FAMILY.into(),

            spacing_1: px(0.0),
            spacing_2: px(0.0),
            spacing_3: px(0.0),
            spacing_4: px(0.0),
            spacing_5: px(0.0),
            spacing_6: px(0.0),
            spacing_8: px(0.0),
            spacing_10: px(0.0),
            spacing_12: px(0.0),
            spacing_16: px(0.0),
            duration_fastest: Duration::ZERO,
            duration_faster: Duration::ZERO,
            duration_fast: Duration::ZERO,
            duration_normal: Duration::ZERO,
            duration_slow: Duration::ZERO,
            duration_slower: Duration::ZERO,
            duration_slowest: Duration::ZERO,
            z_dropdown: 0,
            z_sticky: 0,
            z_modal: 0,
            z_popover: 0,
            z_tooltip: 0,
        }
        .apply_standard()
    }
}

impl ThemeTokens {
    pub fn gradient_primary(&self) -> gpui::Background {
        let c = self.primary;
        let darkened = hsla(c.h, c.s, (c.l - 0.08).max(0.0), c.a);
        gpui::linear_gradient(
            180.0,
            gpui::linear_color_stop(c, 0.0),
            gpui::linear_color_stop(darkened, 1.0),
        )
    }

    pub fn gradient_surface(&self) -> gpui::Background {
        let c = self.card;
        let lightened = hsla(c.h, c.s, (c.l + 0.02).min(1.0), c.a);
        gpui::linear_gradient(
            135.0,
            gpui::linear_color_stop(c, 0.0),
            gpui::linear_color_stop(lightened, 1.0),
        )
    }

    pub fn gradient_accent(&self) -> gpui::Background {
        let c = self.accent;
        let darkened = hsla(c.h, c.s, (c.l - 0.06).max(0.0), c.a);
        gpui::linear_gradient(
            180.0,
            gpui::linear_color_stop(c, 0.0),
            gpui::linear_color_stop(darkened, 1.0),
        )
    }

    pub fn gradient_destructive(&self) -> gpui::Background {
        let c = self.destructive;
        let darkened = hsla(c.h, c.s, (c.l - 0.08).max(0.0), c.a);
        gpui::linear_gradient(
            180.0,
            gpui::linear_color_stop(c, 0.0),
            gpui::linear_color_stop(darkened, 1.0),
        )
    }

    pub fn glow_shadow(&self, color: Hsla, intensity: f32) -> BoxShadow {
        BoxShadow {
            color: color.opacity(0.4 * intensity),
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(20.0 * intensity),
            spread_radius: px(2.0),
            inset: false,
        }
    }

    pub fn colored_shadow(&self, color: Hsla, size: f32) -> BoxShadow {
        BoxShadow {
            color: color.opacity(0.25),
            offset: point(px(0.0), px(2.0 * size)),
            blur_radius: px(8.0 * size),
            spread_radius: px(0.0),
            inset: false,
        }
    }

    pub fn focus_ring_animated(&self, progress: f32) -> BoxShadow {
        BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0 * progress),
            inset: false,
            color: self.ring.opacity(0.5 * progress),
        }
    }

    /// Create a focus ring shadow (3px spread with opacity)
    pub fn focus_ring(&self, opacity: f32) -> BoxShadow {
        BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            inset: false,
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
            inset: false,
            color: self.destructive.opacity(0.2),
        }
    }

    /// Create a success ring (for validated inputs)
    pub fn success_ring(&self) -> BoxShadow {
        let success_color = hsla(0.33, 0.70, 0.50, 1.0);
        BoxShadow {
            offset: point(px(0.0), px(0.0)),
            blur_radius: px(0.0),
            spread_radius: px(3.0),
            inset: false,
            color: success_color.opacity(0.2),
        }
    }

    pub fn inset_shadow_top(&self, intensity: f32) -> BoxShadow {
        BoxShadow {
            offset: point(px(0.0), px(2.0)),
            blur_radius: px(4.0),
            spread_radius: px(-1.0),
            inset: true,
            color: hsla(0.0, 0.0, 0.0, 0.08 * intensity),
        }
    }

    pub fn inset_shadow_bottom(&self, intensity: f32) -> BoxShadow {
        BoxShadow {
            offset: point(px(0.0), px(-2.0)),
            blur_radius: px(4.0),
            spread_radius: px(-1.0),
            inset: true,
            color: hsla(0.0, 0.0, 0.0, 0.08 * intensity),
        }
    }

    pub fn inset_shadow_both(&self, intensity: f32) -> Vec<BoxShadow> {
        vec![
            self.inset_shadow_top(intensity),
            self.inset_shadow_bottom(intensity),
        ]
    }

    pub fn elevation_shadow(&self, level: u8) -> Vec<BoxShadow> {
        match level {
            0 => vec![],
            1 => vec![self.shadow_xs.clone()],
            2 => vec![self.shadow_sm.clone()],
            3 => vec![self.shadow_md.clone()],
            4 => vec![self.shadow_lg.clone()],
            _ => vec![self.shadow_xl.clone()],
        }
    }

    pub fn layered_gradient(&self, angle: f32, colors: &[Hsla]) -> Vec<gpui::Background> {
        if colors.len() < 2 {
            return vec![];
        }

        let mut layers = Vec::new();
        for window in colors.windows(2) {
            layers.push(gpui::linear_gradient(
                angle,
                gpui::linear_color_stop(window[0], 0.0),
                gpui::linear_color_stop(window[1], 1.0),
            ));
        }
        layers
    }
}
