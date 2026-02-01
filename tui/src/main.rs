mod app;
mod config;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io::{stdout, Stdout}};

use app::{App, Popup};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = env::var("ITAD_API_KEY").ok();
    if api_key.is_none() {
        eprintln!("Error: ITAD_API_KEY not set.");
        eprintln!("Create a .env file with:");
        eprintln!("ITAD_API_KEY=your_key_here");
        return Ok(());
    }

    let mut terminal = setup_terminal()?;
    let result = run(&mut terminal, api_key).await;
    restore_terminal()?;
    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

async fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, api_key: Option<String>) -> Result<()> {
    let mut app = App::new(api_key);

    app.load_deals().await;

    // Track when selection changed to debounce game info loading
    let mut last_selection_change = std::time::Instant::now();
    let mut pending_load = false;

    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        if app.should_quit {
            break;
        }

        // Check if we should load game info (after 200ms of no selection change)
        if pending_load && last_selection_change.elapsed() >= std::time::Duration::from_millis(200) {
            pending_load = false;
            app.load_game_info_for_selected().await;
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.popup == Popup::Options {
                        match key.code {
                            KeyCode::Esc => app.close_popup(),
                            KeyCode::Tab | KeyCode::Right => app.options_next_tab(),
                            KeyCode::BackTab | KeyCode::Left => app.options_prev_tab(),
                            KeyCode::Down | KeyCode::Char('j') => app.options_next_item(),
                            KeyCode::Up | KeyCode::Char('k') => app.options_prev_item(),
                            KeyCode::Enter | KeyCode::Char(' ') => app.options_toggle_item(),
                            _ => {}
                        }
                    } else if app.popup == Popup::Keybinds {
                        if key.code == KeyCode::Esc {
                            app.close_popup();
                        }
                    } else if app.show_menu {
                        match key.code {
                            KeyCode::Esc => app.toggle_menu(),
                            KeyCode::Down | KeyCode::Char('j') => app.menu_next(),
                            KeyCode::Up | KeyCode::Char('k') => app.menu_previous(),
                            KeyCode::Enter => {
                                app.menu_select().await;
                            }
                            _ => {}
                        }
                    } else if app.show_platform_dropdown {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('p') => app.toggle_dropdown(),
                            KeyCode::Down | KeyCode::Char('j') => app.dropdown_next(),
                            KeyCode::Up | KeyCode::Char('k') => app.dropdown_previous(),
                            KeyCode::Enter => {
                                app.dropdown_select().await;
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Esc => app.toggle_menu(),
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.next();
                                last_selection_change = std::time::Instant::now();
                                pending_load = true;
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.previous();
                                last_selection_change = std::time::Instant::now();
                                pending_load = true;
                            }
                            KeyCode::Char('p') => app.toggle_dropdown(),
                            KeyCode::Enter => app.open_selected_deal(),
                            KeyCode::Char('r') => {
                                app.load_deals().await;
                                last_selection_change = std::time::Instant::now();
                                pending_load = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
