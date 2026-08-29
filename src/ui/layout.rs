use unicode_width::UnicodeWidthStr;

pub struct TerminalLayout {
    pub width: usize,
}

impl TerminalLayout {
    pub fn compute() -> Self {
        let term_width = crossterm::terminal::size()
            .map(|(w, _)| w as usize)
            .unwrap_or(80);
        let width = term_width.saturating_sub(4).clamp(48, 100);
        Self { width }
    }

    pub fn bidi_isolate(s: &str) -> String {
        format!("\u{2067}{}\u{2069}", s)
    }

    pub fn _pad_line(s: &str, target_width: usize) -> String {
        let visual_w = UnicodeWidthStr::width(s);
        let pad = target_width.saturating_sub(visual_w);
        format!("{}{}", s, " ".repeat(pad))
    }
}
