mod config;
mod context;
mod executor;
mod safety;
mod translator;
mod ui;

use config::Config;
use context::InvocationContext;
use executor::Executor;
use translator::translate;
use ui::{BoxRenderer, InteractivePrompt, UserAction};

fn main() {
    let (cli_opts, ctx) = InvocationContext::capture();
    let mut config = if let Some(ref p) = cli_opts.config_path {
        Config::load_from_path(Some(p))
    } else {
        Config::load()
    };

    if let Some(h) = cli_opts.helper {
        config.general.helper = h;
    }
    if let Some(t) = cli_opts.theme {
        config.general.theme = t;
    }
    if cli_opts.yes {
        config.general.auto_execute = true;
    }

    let mut res = translate(&ctx, &config);

    if cli_opts.json {
        if let Ok(json) = serde_json::to_string_pretty(&res) {
            println!("{}", json);
            return;
        }
    }

    if !ctx.is_interactive {
        eprintln!("[Parch Linux] Translating foreign command:");
        eprintln!("  Suggested command: {}", res.command);
        eprintln!("  Info (EN): {}", res.notes_en);
        eprintln!("  Info (FA): {}", res.notes_fa);
        if let Some(ref w) = res.warning {
            eprintln!("  Warning: {}", w);
        }
        std::process::exit(127);
    }

    BoxRenderer::render(&ctx, &res, &config);

    if cli_opts.dry_run || cli_opts.explain {
        println!("\x1b[1;34mℹ Dry-run / Explain mode: skipping execution.\x1b[0m");
        return;
    }

    if config.general.auto_execute {
        println!("\x1b[1;32m[+] Auto-executing command...\x1b[0m\n");
        if let Err(e) = Executor::run(&ctx, &res) {
            eprintln!("\x1b[1;31m✖ Execution error: {}\x1b[0m", e);
            std::process::exit(1);
        }
        return;
    }

    match InteractivePrompt::prompt(&res) {
        UserAction::Execute(custom_cmd) => {
            if custom_cmd != res.command {
                res.command = custom_cmd;
                let parts: Vec<String> = res.command.split_whitespace().map(|s| s.to_string()).collect();
                if !parts.is_empty() {
                    res.exec_binary = parts[0].clone();
                    res.exec_args = parts[1..].to_vec();
                }
            }

            println!("\x1b[1;32m✔ Executing...\x1b[0m\n");
            if let Err(e) = Executor::run(&ctx, &res) {
                eprintln!("\x1b[1;31m✖ Execution error: {}\x1b[0m", e);
                std::process::exit(1);
            }
        }
        UserAction::Abort => {
            println!("\x1b[1;33m⚠ Operation canceled by user.\x1b[0m");
        }
    }
}
