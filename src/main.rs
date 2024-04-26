use std::{fmt::format, io::stdout, ops::Index};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    widgets::Paragraph,
    Frame, Terminal,
};

struct Screen {
    text: String,
    pos: usize,
}
impl Screen {
    fn new() -> Self {
        Self {
            text: String::new(),
            pos: 0,
        }
    }
    fn left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1
        }
    }
    fn right(&mut self) {
        if self.text.len() == 0 {
            return;
        }
        if self.pos < self.text.len() - 1 {
            self.pos += 1
        }
    }
    fn insert(&mut self, c: char) {
        if self.text.len() == 0 {
            self.text.insert(self.pos, c);
        } else {
            self.text.insert(self.pos + 1, c);
        }
        self.right();
    }
    fn remove(&mut self) {
        if self.pos == 0 && self.text.len() != 0 {
            self.text = String::new();
        }
        if self.pos == 0 {
            return;
        }
        if self.text.len() == self.pos {
            self.text.remove(self.pos - 1);
        } else {
            self.text.remove(self.pos);
        }
        self.left();
    }
    fn pretty(&self) -> String {
        let mut str = vec![];
        for (i, c) in self.text.chars().enumerate() {
            if i == self.pos {
                str.push(format!("[{c}]"))
            } else {
                str.push(format!("{c}"))
            }
        }
        str.join("")
    }
}
fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut t = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut screen = Screen::new();
    let res = run(&mut t, &mut screen);

    disable_raw_mode()?;
    execute!(t.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    t.show_cursor()?;

    res?;
    Ok(())
}
fn run<B: Backend>(t: &mut Terminal<B>, s: &mut Screen) -> anyhow::Result<()> {
    loop {
        t.draw(|f| ui(f, s))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => {
                    // laskdjflsk
                    s.insert(c);
                }
                KeyCode::Backspace => {
                    //alsdkjfslkdjfl
                    s.remove();
                }
                KeyCode::Left => {
                    //sldjflskdjf
                    s.left();
                }
                KeyCode::Right => {
                    //lsdkjfsldkjfls
                    s.right();
                }
                _ => {}
            }
        }
    }
    Ok(())
}
fn ui(f: &mut Frame, screen: &Screen) {
    let text = Paragraph::new(format!("pos {}. text {}", screen.pos, screen.pretty()));

    f.render_widget(text, f.size())
}
