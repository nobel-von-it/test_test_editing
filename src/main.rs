use std::io::stdout;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::Stylize,
    widgets::Paragraph,
    Frame, Terminal,
};

struct Screen {
    // вектор чаров проще в изменении, чем строка
    text: Vec<char>,
    pos: usize,
}
impl Screen {
    fn new() -> Self {
        Self {
            // создать массив нужно с пустым чар символом, чтобы сместить все остальные буквы на 1
            text: vec!['\0'],
            pos: 0,
        }
    }

    fn left(&mut self) {
        // нужна только проверка недопускающая отрицательного значения
        if self.pos > 0 {
            self.pos -= 1
        }
    }

    fn right(&mut self) {
        // здесь немного посложнее
        // нужно учесть, что позиция меньше длины массива и что сам массив не пуст
        // потому что позиция может быть 0 и длина массива 0 и при сравнении 0 > -1, что приведёт к ошибке
        if self.pos < self.text.len() - 1 && self.text.len() != 0 {
            self.pos += 1
        }
    }
    fn insert(&mut self, c: char) {
        // просто вставляем символ на следующую позицию после псевдокурсора
        // и сдвигаем позицию вправо
        self.text.insert(self.pos + 1, c);
        self.right();
    }
    fn remove(&mut self) {
        // так как настоящий текст начинается с 1 символа, чтобы можно было перейти в начало и писать оттуда
        // проверка на то, что длина массива больше 1. Позиция также должна быть больше 0, потому что иначе
        // удалится 0-ой символ и всё сломается
        if self.text.len() > 1 && self.pos > 0 {
            self.text.remove(self.pos);
            self.left();
        }
    }
    // может быть, неоптимизированная функция для получения строки с добавленным прсевдокурсором
    fn pretty(&self) -> String {
        // первоначальная реализация псевдокурсора
        // let mut str = vec![];
        // for (i, c) in self.text.iter().enumerate() {
        //     if i == self.pos {
        //         str.push(format!("{c}|"))
        //     } else {
        //         str.push(format!("{c}"))
        //     }
        // }
        // str.join("")

        // так как в книге по расту разработчики говорят, что итераторы быстрее циклов
        // и что лучше при любой возможности использовать их, я использую именно итератор
        self.text
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == self.pos {
                    format!("{c}|")
                } else {
                    format!("{c}")
                }
            })
            .collect::<Vec<String>>()
            .join("")
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
                KeyCode::Char(c) => s.insert(c),
                KeyCode::Backspace => s.remove(),
                KeyCode::Left => s.left(),
                KeyCode::Right => s.right(),
                _ => {}
            }
        }
    }
    Ok(())
}
fn ui(f: &mut Frame, screen: &Screen) {
    // виджет для тестов с отображением позиции
    // let text = Paragraph::new(format!(
    //     "pos: {} with text: {}",
    //     screen.pos,
    //     screen.pretty()
    // ))
    // .centered()
    // .on_dark_gray();

    // виджет для релиза без позиции
    let text = Paragraph::new(screen.pretty()).centered().on_dark_gray();

    f.render_widget(text, f.size())
}
