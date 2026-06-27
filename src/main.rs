mod app;
mod confetti;
mod ui;
mod words;

use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::cursor::SetCursorStyle;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;

use app::{App, Source};

const WORD_COUNT: usize = 25;

fn main() -> io::Result<()> {
    let source = match parse_args() {
        Some(source) => source,
        None => return Ok(()),
    };

    let mut terminal = setup()?;
    let result = run(&mut terminal, source);
    restore(&mut terminal)?;
    result
}

fn parse_args() -> Option<Source> {
    let mut args = std::env::args().skip(1);
    let mut words = WORD_COUNT;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return None;
            }
            "-t" | "--text" => {
                if let Some(text) = args.next() {
                    return Some(Source::Fixed(text));
                }
            }
            "-w" | "--words" => {
                if let Some(value) = args.next().and_then(|v| v.parse().ok()) {
                    words = value;
                }
            }
            _ => {}
        }
    }

    Some(Source::Random(words))
}

fn print_help() {
    println!("verve — a minimalist typing speed test for the terminal");
    println!();
    println!("Usage: verve [options]");
    println!();
    println!("Options:");
    println!("  -w, --words <n>     number of random words (default: {WORD_COUNT})");
    println!("  -t, --text <text>   type a fixed text instead of random words");
    println!("  -h, --help          show this help");
    println!();
    println!("Controls:");
    println!("  tab    restart        ctrl+backspace   delete word");
    println!("  esc    quit           backspace        delete");
}

fn setup() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, SetCursorStyle::BlinkingBar)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore<B: Backend + io::Write>(terminal: &mut Terminal<B>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        SetCursorStyle::DefaultUserShape
    )?;
    terminal.show_cursor()
}

fn run<B: Backend>(terminal: &mut Terminal<B>, source: Source) -> io::Result<()> {
    let mut app = App::new(source);

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        let tick = if app.celebration_elapsed().is_some() {
            33
        } else if app.is_started() && !app.is_finished() {
            100
        } else {
            200
        };

        if !event::poll(Duration::from_millis(tick))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
            let alt = key.modifiers.contains(KeyModifiers::ALT);

            match key.code {
                KeyCode::Char('c') if ctrl => break,
                KeyCode::Esc => break,
                KeyCode::Tab => app.reset(),
                KeyCode::Backspace if ctrl || alt => app.delete_word(),
                KeyCode::Char('w' | 'h' | '_') if ctrl => app.delete_word(),
                KeyCode::Char('\u{8}' | '\u{17}' | '\u{1f}') => app.delete_word(),
                KeyCode::Backspace => app.backspace(),
                KeyCode::Char(c) if !ctrl && !alt && !c.is_control() => app.push(c),
                _ => {}
            }
        }
    }

    Ok(())
}
