use colored::Colorize;

pub struct Theme;

impl Theme {
    pub fn title(text: &str) -> String {
        text.bold().bright_blue().to_string()
    }

    pub fn logo_bracket(text: &str) -> String {
        text.bold().bright_cyan().to_string()
    }

    pub fn input_label(text: &str) -> String {
        text.bold().bright_yellow().to_string()
    }

    pub fn input_val(text: &str) -> String {
        text.white().bold().to_string()
    }

    pub fn target_label(text: &str) -> String {
        text.bold().bright_green().to_string()
    }

    pub fn target_val(text: &str) -> String {
        text.bold().bright_green().to_string()
    }

    pub fn badge_en() -> String {
        " EN ".on_bright_blue().bold().black().to_string()
    }

    pub fn badge_fa() -> String {
        " FA ".on_bright_green().bold().black().to_string()
    }

    pub fn note_en(text: &str) -> String {
        text.bright_white().to_string()
    }

    pub fn note_fa(text: &str) -> String {
        text.bright_cyan().to_string()
    }

    pub fn border(text: &str) -> String {
        text.blue().to_string()
    }

    pub fn warning(text: &str) -> String {
        text.bright_red().bold().to_string()
    }
}
