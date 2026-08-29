#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeStyle {
    Neon,
    ParchDark,
    Minimal,
    Monokai,
    Plain,
}

impl ThemeStyle {
    pub fn parse(s: &str) -> Self {
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
            ThemeStyle::Neon => format!("\x1b[1;38;5;51m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[1;38;5;201m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[1;38;5;231m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[1;38;5;220m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn logo_bracket(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[1;38;5;39m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[1;38;5;201m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[38;5;231m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[1;38;5;197m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn input_label(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[1;38;5;220m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[1;38;5;51m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[1;38;5;231m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[1;38;5;81m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn input_val(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[1;38;5;231m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[38;5;231m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[38;5;250m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[38;5;148m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn target_label(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[1;38;5;48m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[1;38;5;48m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[1;38;5;231m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[1;38;5;141m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn target_val(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[1;4;38;5;48m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[1;38;5;51m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[1;4;38;5;231m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[1;4;38;5;220m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn badge_en(style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => "\x1b[38;5;39m❬\x1b[1;38;5;51mEN\x1b[0;38;5;39m❭\x1b[0m".to_string(),
            ThemeStyle::ParchDark => "\x1b[38;5;201m❬\x1b[1;38;5;51mEN\x1b[0;38;5;201m❭\x1b[0m".to_string(),
            ThemeStyle::Minimal => "\x1b[1;38;5;231m[EN]\x1b[0m".to_string(),
            ThemeStyle::Monokai => "\x1b[38;5;141m❬\x1b[1;38;5;81mEN\x1b[0;38;5;141m❭\x1b[0m".to_string(),
            ThemeStyle::Plain => "[EN]".to_string(),
        }
    }

    pub fn badge_fa(style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => "\x1b[38;5;39m❬\x1b[1;38;5;48mFA\x1b[0;38;5;39m❭\x1b[0m".to_string(),
            ThemeStyle::ParchDark => "\x1b[38;5;201m❬\x1b[1;38;5;220mFA\x1b[0;38;5;201m❭\x1b[0m".to_string(),
            ThemeStyle::Minimal => "\x1b[1;38;5;231m[FA]\x1b[0m".to_string(),
            ThemeStyle::Monokai => "\x1b[38;5;141m❬\x1b[1;38;5;220mFA\x1b[0;38;5;141m❭\x1b[0m".to_string(),
            ThemeStyle::Plain => "[FA]".to_string(),
        }
    }

    pub fn note_en(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[38;5;231m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[38;5;231m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[38;5;250m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[38;5;231m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn note_fa(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[38;5;51m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[38;5;220m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[38;5;250m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[38;5;81m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn border(text: &str, style: ThemeStyle) -> String {
        match style {
            ThemeStyle::Neon => format!("\x1b[38;5;39m{}\x1b[0m", text),
            ThemeStyle::ParchDark => format!("\x1b[38;5;201m{}\x1b[0m", text),
            ThemeStyle::Minimal => format!("\x1b[38;5;244m{}\x1b[0m", text),
            ThemeStyle::Monokai => format!("\x1b[38;5;220m{}\x1b[0m", text),
            ThemeStyle::Plain => text.to_string(),
        }
    }

    pub fn warning(text: &str) -> String {
        format!("\x1b[1;38;5;196m{}\x1b[0m", text)
    }
}
