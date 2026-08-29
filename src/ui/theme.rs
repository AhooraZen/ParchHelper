use colored::Colorize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeStyle {
    Neon,
    ParchDark,
    Minimal,
    Monokai,
    Plain,
}

impl ThemeStyle {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "neon" | "cyberpunk" => ThemeStyle::Neon,
            "dark" | "parch" | "parch-dark" => ThemeStyle::ParchDark,
            "minimal" | "clean" => ThemeStyle::Minimal,
            "monokai" => ThemeStyle::Monokai,
            "plain" | "none" | "no-color" => ThemeStyle::Plain,
            _ => ThemeStyle::Neon,
        }
    }
}

pub struct Theme;

impl Theme {
    pub fn title(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bold().bright_cyan().to_string(),
            ThemeStyle::ParchDark => text.bold().bright_magenta().to_string(),
            ThemeStyle::Minimal => text.bold().white().to_string(),
            ThemeStyle::Monokai => text.bold().bright_yellow().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn logo_bracket(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bold().bright_blue().to_string(),
            ThemeStyle::ParchDark => text.bold().bright_purple().to_string(),
            ThemeStyle::Minimal => text.white().to_string(),
            ThemeStyle::Monokai => text.bold().bright_red().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn input_label(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bold().bright_yellow().to_string(),
            ThemeStyle::ParchDark => text.bold().bright_cyan().to_string(),
            ThemeStyle::Minimal => text.bold().white().to_string(),
            ThemeStyle::Monokai => text.bold().bright_blue().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn input_val(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bright_white().bold().to_string(),
            ThemeStyle::ParchDark => text.bright_white().to_string(),
            ThemeStyle::Minimal => text.white().to_string(),
            ThemeStyle::Monokai => text.bright_green().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn target_label(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bold().bright_green().to_string(),
            ThemeStyle::ParchDark => text.bold().bright_green().to_string(),
            ThemeStyle::Minimal => text.bold().white().to_string(),
            ThemeStyle::Monokai => text.bold().bright_purple().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn target_val(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bold().bright_green().underline().to_string(),
            ThemeStyle::ParchDark => text.bold().bright_cyan().to_string(),
            ThemeStyle::Minimal => text.bold().white().underline().to_string(),
            ThemeStyle::Monokai => text.bold().bright_yellow().underline().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn badge_en(style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => " EN ".on_cyan().bold().black().to_string(),
            ThemeStyle::ParchDark => " EN ".on_purple().bold().white().to_string(),
            ThemeStyle::Minimal => "[EN]".bold().white().to_string(),
            ThemeStyle::Monokai => " EN ".on_blue().bold().black().to_string(),
            ThemeStyle::Plain => "[EN]".to_string(),
        }
    }

    pub fn badge_fa(style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => " FA ".on_green().bold().black().to_string(),
            ThemeStyle::ParchDark => " FA ".on_blue().bold().white().to_string(),
            ThemeStyle::Minimal => "[FA]".bold().white().to_string(),
            ThemeStyle::Monokai => " FA ".on_yellow().bold().black().to_string(),
            ThemeStyle::Plain => "[FA]".to_string(),
        }
    }

    pub fn note_en(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bright_white().to_string(),
            ThemeStyle::ParchDark => text.bright_white().to_string(),
            ThemeStyle::Minimal => text.white().to_string(),
            ThemeStyle::Monokai => text.bright_white().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn note_fa(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bright_cyan().to_string(),
            ThemeStyle::ParchDark => text.bright_yellow().to_string(),
            ThemeStyle::Minimal => text.white().to_string(),
            ThemeStyle::Monokai => text.bright_cyan().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn border(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => text.bright_blue().to_string(),
            ThemeStyle::ParchDark => text.bright_purple().to_string(),
            ThemeStyle::Minimal => text.bright_black().to_string(),
            ThemeStyle::Monokai => text.bright_yellow().to_string(),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn warning(text: &str) -> String {
        text.bright_red().bold().to_string()
    }
}
