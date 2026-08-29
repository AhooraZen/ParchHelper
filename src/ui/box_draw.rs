use super::layout::TerminalLayout;
use super::theme::{Theme, ThemeStyle};
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

        let theme_str = ctx
            .cli_opts
            .theme
            .as_deref()
            .unwrap_or(&config.general.theme);
        let style = ThemeStyle::parse(theme_str);

        if style == ThemeStyle::Plain {
            Self::render_plain(ctx, res, config);
            return;
        }

        let layout = TerminalLayout::compute();
        let input_cmd = format!(
            "{} {}",
            ctx.source.display_name(),
            ctx.original_args.join(" ")
        );

        let title = "Parch Linux Command Helper";
        let top_border = format!(
            "╭─── {} {} {} {}",
            Theme::logo_bracket("❬", style),
            Theme::title(title, style),
            Theme::logo_bracket("❭", style),
            Theme::border(&"─".repeat(layout.width.saturating_sub(38)), style)
        );
        let bot_border = format!("╰{}╯", "─".repeat(layout.width));

        println!();
        println!("{}", Theme::border(&top_border, style));
        println!("{}", Theme::border("│", style));
        println!(
            "{}   {:<12} : {}",
            Theme::border("│", style),
            Theme::input_label("Input", style),
            Theme::input_val(&input_cmd, style)
        );
        println!(
            "{}   {:<12} : {}",
            Theme::border("│", style),
            Theme::target_label("Arch/Parch", style),
            Theme::target_val(&res.command, style)
        );
        println!("{}", Theme::border("│", style));

        let lang = config.general.language.as_str();
        if lang == "both" || lang == "en" {
            println!(
                "{}   {}  {}",
                Theme::border("│", style),
                Theme::badge_en(style),
                Theme::note_en(&res.notes_en, style)
            );
        }
        if lang == "both" || lang == "fa" {
            let note_fa_rendered = if config.general.bidi_isolation {
                TerminalLayout::bidi_isolate(&res.notes_fa)
            } else {
                res.notes_fa.clone()
            };

            println!(
                "{}   {}  {}",
                Theme::border("│", style),
                Theme::badge_fa(style),
                Theme::note_fa(&note_fa_rendered, style)
            );
        }

        if let Some(ref warn) = res.warning {
            println!("{}", Theme::border("│", style));
            println!(
                "{}   {:<12} : {}",
                Theme::border("│", style),
                Theme::warning("Warning"),
                Theme::warning(warn)
            );
        }

        println!("{}", Theme::border("│", style));
        println!("{}", Theme::border(&bot_border, style));
        println!();
    }

    fn render_plain(ctx: &InvocationContext, res: &TranslationResult, _config: &Config) {
        let input_cmd = format!(
            "{} {}",
            ctx.source.display_name(),
            ctx.original_args.join(" ")
        );
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
