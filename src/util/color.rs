use owo_colors::{Style};
use std::borrow::Cow;
use std::env;

#[derive(Clone, Copy)]
pub struct ColorConfig {
    enabled: bool,
}

impl ColorConfig {
    pub fn from_env_and_flag(no_colors_flag: bool) -> Self {
        if no_colors_flag {
            return Self { enabled: false };
        }

        let env_var = env::var("GIT_NOPE_COLORS").unwrap_or_else(|_| "true".to_string());
        let enabled = match env_var.to_lowercase().as_str() {
            "false" | "0" | "no" | "off" | "" => false,
            _ => true,
        };

        Self { enabled }
    }

    pub fn paint<'a>(&self, style: Style, text: impl Into<Cow<'a, str>>) -> Cow<'a, str> {
        if self.enabled {
            let text = text.into();
            // We use format! because owo_colors types don't return Cow, but Display impls.
            // This is a bit of a tradeoff for simplicity.
            Cow::Owned(style.style(text).to_string())
        } else {
            text.into()
        }
    }

    pub fn red_style(&self) -> Style {
        Style::new().red()
    }

    pub fn green_style(&self) -> Style {
        Style::new().green()
    }

    pub fn yellow_style(&self) -> Style {
        Style::new().yellow()
    }
    
    pub fn cyan_style(&self) -> Style {
        Style::new().cyan()
    }
    
    pub fn dim_style(&self) -> Style {
         Style::new().dimmed()
    }
}
