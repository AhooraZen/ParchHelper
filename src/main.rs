mod config;
mod context;
mod executor;
mod safety;
mod translator;
mod ui;

use config::Config;
use context::InvocationContext;
use executor::Executor;
use std::io::{self, Write};
use translator::translate;
use ui::BoxRenderer;

fn main() {
    let ctx = InvocationContext::capture();
    let config = Config::load();

    let res = translate(&ctx, &config);

    if !ctx.is_interactive {
        eprintln!("[Parch Linux] Translating foreign command:");
        eprintln!("  Suggested command: {}", res.command);
        eprintln!("  Info (EN): {}", res.notes_en);
        eprintln!("  Info (FA): {}", res.notes_fa);
        std::process::exit(127);
    }

    BoxRenderer::render(&ctx, &res, &config);

    if config.general.auto_execute {
        println!("\x1b[1;32m[+] Auto-executing command...\x1b[0m\n");
        if let Err(e) = Executor::run(&ctx, &res) {
            eprintln!("Execution error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    print!("\x1b[1;36m╭─▶ \x1b[1;37mExecute \x1b[1;32m`{}`\x1b[1;37m ? \x1b[1;33m[Y/n/c]\x1b[1;36m ❯ \x1b[0m", res.command);
    io::stdout().flush().unwrap();

    let mut user_choice = String::new();
    if io::stdin().read_line(&mut user_choice).is_ok() {
        let clean = user_choice.trim().to_lowercase();
        if clean.is_empty() || clean == "y" || clean == "yes" {
            println!("\x1b[1;32m✔ Executing...\x1b[0m\n");
            if let Err(e) = Executor::run(&ctx, &res) {
                eprintln!("\x1b[1;31m✖ Execution error: {}\x1b[0m", e);
                std::process::exit(1);
            }
        } else {
            println!("\x1b[1;33m⚠ Operation canceled by user.\x1b[0m");
        }
    }
}
