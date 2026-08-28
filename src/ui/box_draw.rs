use super::theme::Theme;
use crate::config::Config;
use crate::context::InvocationContext;
use crate::translator::TranslationResult;

pub struct BoxRenderer;

impl BoxRenderer {
    pub fn render(ctx: &InvocationContext, res: &TranslationResult, config: &Config) {
        if !config.general.colored_ui {
            Self::render_plain(ctx, res, config);
            return;
        }

        let input_cmd = format!("{} {}", ctx.source.display_name(), ctx.original_args.join(" "));
        let width: usize = 76;

        let top_border = format!(
            "╭─ {} {} {} {}",
            Theme::logo_bracket("["),
            Theme::title("Parch Linux Command Helper"),
            Theme::logo_bracket("]"),
            Theme::border(&"─".repeat(width.saturating_sub(36)))
        );
        let bot_border = format!("╰{}╯", "─".repeat(width));

        println!();
        println!("{}", Theme::border(&top_border));
        println!("{}", Theme::border("│"));
        println!("{}   {:<12} : {}", Theme::border("│"), Theme::input_label("Input"), Theme::input_val(&input_cmd));
        println!("{}   {:<12} : {}", Theme::border("│"), Theme::target_label("Arch/Parch"), Theme::target_val(&res.command));
        println!("{}", Theme::border("│"));

        let lang = config.general.language.as_str();
        if lang == "both" || lang == "en" {
            println!("{}   {}  {}", Theme::border("│"), Theme::badge_en(), Theme::note_en(&res.notes_en));
        }
        if lang == "both" || lang == "fa" {
            println!("{}   {}  {}", Theme::border("│"), Theme::badge_fa(), Theme::note_fa(&res.notes_fa));
        }

        if let Some(ref warn) = res.warning {
            println!("{}", Theme::border("│"));
            println!("{}   {:<12} : {}", Theme::border("│"), Theme::warning("Warning"), Theme::warning(warn));
        }

        println!("{}", Theme::border("│"));
        println!("{}", Theme::border(&bot_border));
        println!();
    }

    fn render_plain(ctx: &InvocationContext, res: &TranslationResult, _config: &Config) {
        let input_cmd = format!("{} {}", ctx.source.display_name(), ctx.original_args.join(" "));
        println!("\n==================================================");
        println!("Input:       {}", input_cmd);
        println!("Arch/Parch:  {}", res.command);
        println!("EN: {}", res.notes_en);
        println!("FA: {}", res.notes_fa);
        if let Some(ref warn) = res.warning {
            println!("Warning: {}", warn);
        }
        println!("==================================================\n");
    }
}
