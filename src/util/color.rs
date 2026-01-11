use std::env;

use std::borrow::Cow;

use owo_colors::{OwoColorize, Style};

#[derive(Clone, Copy, Debug)]
pub struct ColorConfig {
    enabled: bool,
}

impl ColorConfig {
    pub fn from_env_and_flag(disable_flag: bool) -> Self {
        if disable_flag {
            return Self { enabled: false };
        }

        let env_setting = env::var("GIT_NOPE_COLORS").ok();
        let enabled = match env_setting.as_deref() {
            Some(val) if matches_false(val) => false,
            _ => true,
        };

        Self { enabled }
    }

    pub fn paint<'a>(&self, style: Style, text: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
        if self.enabled {
            let cow = text.into();
            Cow::Owned(cow.into_owned().style(style).to_string())
        } else {
            text.into()
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

fn matches_false(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "0" | "false" | "no" | "off"
    )
}

pub fn red_style() -> Style {
    Style::new().red()
}

pub fn green_style() -> Style {
    Style::new().green()
}

pub fn yellow_style() -> Style {
    Style::new().yellow()
}
