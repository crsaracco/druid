// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Theme keys and initial values.

use crate::piet::Color;

use crate::{Env, Key};

pub const WINDOW_BACKGROUND_COLOR: Key<Color> = Key::new("window_background_color");

pub const LABEL_COLOR: Key<Color> = Key::new("label_color");
pub const PLACEHOLDER_COLOR: Key<Color> = Key::new("placeholder_color");

pub const PRIMARY_LIGHT: Key<Color> = Key::new("primary_light");
pub const PRIMARY_DARK: Key<Color> = Key::new("primary_dark");
pub const PROGRESS_BAR_RADIUS: Key<f64> = Key::new("progress_bar_radius");
pub const BACKGROUND_LIGHT: Key<Color> = Key::new("background_light");
pub const BACKGROUND_DARK: Key<Color> = Key::new("background_dark");
pub const FOREGROUND_LIGHT: Key<Color> = Key::new("foreground_light");
pub const FOREGROUND_DARK: Key<Color> = Key::new("foreground_dark");
pub const BUTTON_DARK: Key<Color> = Key::new("button_dark");
pub const BUTTON_LIGHT: Key<Color> = Key::new("button_light");
pub const BUTTON_BORDER_RADIUS: Key<f64> = Key::new("button_radius");
pub const BUTTON_BORDER_WIDTH: Key<f64> = Key::new("button_border_width");
pub const BORDER_DARK: Key<Color> = Key::new("border");
pub const BORDER_LIGHT: Key<Color> = Key::new("border_light");
pub const SELECTION_COLOR: Key<Color> = Key::new("selection_color");
pub const CURSOR_COLOR: Key<Color> = Key::new("cursor_color");

pub const FONT_NAME: Key<&str> = Key::new("font_name");
pub const TEXT_SIZE_NORMAL: Key<f64> = Key::new("text_size_normal");
pub const TEXT_SIZE_LARGE: Key<f64> = Key::new("text_size_large");
pub const BASIC_WIDGET_HEIGHT: Key<f64> = Key::new("basic_widget_height");

/// The default minimum width for a 'wide' widget; a textbox, slider, progress bar, etc.
pub const WIDE_WIDGET_WIDTH: Key<f64> = Key::new("druid.widgets.long-widget-width");
pub const BORDERED_WIDGET_HEIGHT: Key<f64> = Key::new("bordered_widget_height");

pub const TEXTBOX_BORDER_RADIUS: Key<f64> = Key::new("textbox_radius");

pub const SCROLLBAR_COLOR: Key<Color> = Key::new("scrollbar_color");
pub const SCROLLBAR_BORDER_COLOR: Key<Color> = Key::new("scrollbar_border_color");
pub const SCROLLBAR_MAX_OPACITY: Key<f64> = Key::new("scrollbar_max_opacity");
pub const SCROLLBAR_FADE_DELAY: Key<u64> = Key::new("scrollbar_fade_time");
pub const SCROLLBAR_WIDTH: Key<f64> = Key::new("scrollbar_width");
pub const SCROLLBAR_PAD: Key<f64> = Key::new("scrollbar_pad");
pub const SCROLLBAR_RADIUS: Key<f64> = Key::new("scrollbar_radius");
pub const SCROLLBAR_EDGE_WIDTH: Key<f64> = Key::new("scrollbar_edge_width");

/// An initial theme.
pub fn init() -> Env {
    let mut env = Env::default()
        .adding(WINDOW_BACKGROUND_COLOR, Color::rgb8(0x29, 0x29, 0x29))
        .adding(LABEL_COLOR, Color::rgb8(0xf0, 0xf0, 0xea))
        .adding(PLACEHOLDER_COLOR, Color::rgb8(0x80, 0x80, 0x80))
        .adding(PRIMARY_LIGHT, Color::rgb8(0x5c, 0xc4, 0xff))
        .adding(PRIMARY_DARK, Color::rgb8(0x00, 0x8d, 0xdd))
        .adding(PROGRESS_BAR_RADIUS, 4.)
        .adding(BACKGROUND_LIGHT, Color::rgb8(0x3a, 0x3a, 0x3a))
        .adding(BACKGROUND_DARK, Color::rgb8(0x31, 0x31, 0x31))
        .adding(FOREGROUND_LIGHT, Color::rgb8(0xf9, 0xf9, 0xf9))
        .adding(FOREGROUND_DARK, Color::rgb8(0xbf, 0xbf, 0xbf))
        .adding(BUTTON_DARK, Color::BLACK)
        .adding(BUTTON_LIGHT, Color::rgb8(0x21, 0x21, 0x21))
        .adding(BUTTON_BORDER_RADIUS, 4.)
        .adding(BUTTON_BORDER_WIDTH, 2.)
        .adding(BORDER_DARK, Color::rgb8(0x3a, 0x3a, 0x3a))
        .adding(BORDER_LIGHT, Color::rgb8(0xa1, 0xa1, 0xa1))
        .adding(SELECTION_COLOR, Color::rgb8(0xf3, 0x00, 0x21))
        .adding(CURSOR_COLOR, Color::WHITE)
        .adding(TEXT_SIZE_NORMAL, 15.0)
        .adding(TEXT_SIZE_LARGE, 24.0)
        .adding(BASIC_WIDGET_HEIGHT, 18.0)
        .adding(WIDE_WIDGET_WIDTH, 100.)
        .adding(BORDERED_WIDGET_HEIGHT, 24.0)
        .adding(TEXTBOX_BORDER_RADIUS, 2.)
        .adding(SCROLLBAR_COLOR, Color::rgb8(0xff, 0xff, 0xff))
        .adding(SCROLLBAR_BORDER_COLOR, Color::rgb8(0x77, 0x77, 0x77))
        .adding(SCROLLBAR_MAX_OPACITY, 0.7)
        .adding(SCROLLBAR_FADE_DELAY, 1500u64)
        .adding(SCROLLBAR_WIDTH, 8.)
        .adding(SCROLLBAR_PAD, 2.)
        .adding(SCROLLBAR_RADIUS, 5.)
        .adding(SCROLLBAR_EDGE_WIDTH, 1.);

    #[cfg(target_os = "windows")]
    {
        env = env.adding(FONT_NAME, "Segoe UI");
    }
    #[cfg(target_os = "macos")]
    {
        // Ideally this would be a reference to San Francisco, but Cairo's
        // "toy text" API doesn't seem to be able to access it easily.
        env = env.adding(FONT_NAME, "Arial");
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        env = env.adding(FONT_NAME, "sans-serif");
    }
    env
}
