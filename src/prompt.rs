use crate::app::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, execute, terminal};

pub enum PromptResult<'a> {
    Noop,
    Exit,
    Value(&'a str),
}

#[derive(Default)]
pub struct Prompt {
    prompt: String,
    input: String,
    cursor_pos: usize,
}

impl Prompt {
    pub fn new<S: Into<String>>(prompt: S) -> Self {
        Self {
            prompt: prompt.into(),
            ..Default::default()
        }
    }

    pub fn print_prompt(&self) -> Result {
        let stdout = std::io::stdout();
        execute!(
            &mut stdout.lock(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
        )?;
        print!("{}", self.prompt);
        std::io::Write::flush(&mut stdout.lock())?;
        Ok(())
    }

    pub fn next_line(&self) -> Result {
        let stdout = std::io::stdout();
        println!();
        execute!(
            &mut stdout.lock(),
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
        )?;
        std::io::Write::flush(&mut stdout.lock())?;
        Ok(())
    }

    pub fn handle_event(&mut self, event: KeyEvent) -> Result<PromptResult> {
        use PromptResult::*;

        let stdout = std::io::stdout();
        let flush = || std::io::Write::flush(&mut stdout.lock());

        match (event.code, event.modifiers) {
            // QUIT
            (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                Ok(Exit)
            }

            // BACKSPACE
            (KeyCode::Backspace, _) => {
                if !self.input.is_empty() {
                    self.input.pop();
                    self.cursor_pos =
                        self.cursor_pos.checked_sub(1).unwrap_or_default();
                    execute!(
                        &mut stdout.lock(),
                        cursor::MoveLeft(1),
                        terminal::Clear(terminal::ClearType::UntilNewLine),
                    )?;
                    flush()?;
                }

                Ok(Value(&self.input))
            }

            // CLEAR
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                self.input.clear();
                self.cursor_pos = 0;
                execute!(
                    &mut stdout.lock(),
                    cursor::MoveToColumn(self.prompt.len() as u16),
                    terminal::Clear(terminal::ClearType::UntilNewLine),
                )?;
                flush()?;

                Ok(Value(&self.input))
            }

            // ARROW KEY NAVIGATION
            (KeyCode::Left, _) => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                    execute!(&mut stdout.lock(), cursor::MoveLeft(1))?;
                    flush()?;
                }

                Ok(Noop)
            }

            (KeyCode::Right, _) => {
                if self.cursor_pos < self.input.len() {
                    self.cursor_pos += 1;
                    execute!(&mut stdout.lock(), cursor::MoveRight(1))?;
                    flush()?;
                }

                Ok(Noop)
            }

            // INPUT
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                self.input.push(c);
                self.cursor_pos += 1;
                print!("{}", c);
                flush()?;

                Ok(Value(&self.input))
            }

            _ => Ok(Noop),
        }
    }
}
