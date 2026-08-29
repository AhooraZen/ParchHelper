use crate::translator::TranslationResult;
use base64::Engine;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io::{stdout, Write};

pub enum UserAction {
    Execute(String),
    Abort,
}

pub struct InteractivePrompt;

impl InteractivePrompt {
    pub fn prompt(res: &TranslationResult) -> UserAction {
        print!("\x1b[1;36m╭─▶ \x1b[1;37mExecute \x1b[1;32m`{}`\x1b[1;37m ? \x1b[1;33m[Enter/y: Run | c: Copy | e: Edit | q: Abort]\x1b[1;36m ❯ \x1b[0m", res.command);
        stdout().flush().unwrap();

        if let Ok(()) = crossterm::terminal::enable_raw_mode() {
            let action = loop {
                if let Ok(Event::Key(key_event)) = event::read() {
                    match key_event.code {
                        KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                            break UserAction::Execute(res.command.clone());
                        }
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                                break UserAction::Abort;
                            } else {
                                Self::copy_to_clipboard(&res.command);
                            }
                        }
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            let _ = crossterm::terminal::disable_raw_mode();
                            println!();
                            let edited = Self::prompt_edit(&res.command);
                            return UserAction::Execute(edited);
                        }
                        KeyCode::Char('q') | KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            break UserAction::Abort;
                        }
                        _ => {}
                    }
                }
            };

            let _ = crossterm::terminal::disable_raw_mode();
            println!();
            action
        } else {
            // Fallback to standard line-buffered stdin
            let mut line = String::new();
            if std::io::stdin().read_line(&mut line).is_ok() {
                let clean = line.trim().to_lowercase();
                if clean.is_empty() || clean == "y" || clean == "yes" {
                    UserAction::Execute(res.command.clone())
                } else if clean == "c" || clean == "copy" {
                    Self::copy_to_clipboard(&res.command);
                    UserAction::Abort
                } else if clean == "e" || clean == "edit" {
                    let edited = Self::prompt_edit(&res.command);
                    UserAction::Execute(edited)
                } else {
                    UserAction::Abort
                }
            } else {
                UserAction::Abort
            }
        }
    }

    fn copy_to_clipboard(text: &str) {
        let b64 = base64::engine::general_purpose::STANDARD.encode(text);
        print!("\x1b]52;c;{}\x07", b64);
        let _ = stdout().flush();
        println!("\r\x1b[1;32m✔ Copied to clipboard via OSC-52!\x1b[0m");
        print!("\x1b[1;36m╭─▶ \x1b[1;37mExecute ? \x1b[1;33m[Enter/y: Run | e: Edit | q: Abort]\x1b[1;36m ❯ \x1b[0m");
        let _ = stdout().flush();
    }

    fn prompt_edit(initial: &str) -> String {
        print!("\x1b[1;33mEdit command:\x1b[0m ");
        let _ = stdout().flush();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        initial.to_string()
    }
}
