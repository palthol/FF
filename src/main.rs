/**
 * rust-ff: A terminal-based file finder utility
 * 
 * This application provides a TUI for quickly finding and opening files.
 * It uses ratatui for the interface and allows for real-time searching
 * with keyboard navigation.
 */


mod file_finder;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_finder::FileFinder;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::{env, io, path::PathBuf, time::{Duration, Instant}}; // Added time imports

/**
 * Main application state structure
 * Holds the current search keyword, matched files, and UI state
 */

struct App {
    keyword: String,
    files: Vec<PathBuf>,
    selected_index: usize,
    show_help: bool,
}

impl App {
    fn new(initial_keyword: String) -> Self {
        App {
            keyword: initial_keyword,
            files: Vec::new(),
            selected_index: 0,
            show_help: false,
        }
    }

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    fn update_files(&mut self, finder: &FileFinder) {
        self.files = finder.find_files(&self.keyword);
        self.selected_index = 0;
    }

    fn add_char(&mut self, c: char, finder: &FileFinder) {
        self.keyword.push(c);
        self.update_files(finder);
    }

    fn remove_char(&mut self, finder: &FileFinder) {
        self.keyword.pop();
        self.update_files(finder);
    }

    fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_selection_down(&mut self) {
        if !self.files.is_empty() && self.selected_index + 1 < self.files.len() {
            self.selected_index += 1;
        }
    }

    fn open_selected_file(&self, finder: &FileFinder) -> Result<(), std::io::Error> {
        if let Some(file) = self.files.get(self.selected_index) {
            finder.open_in_vscode(file)?;
        }
        Ok(())
    }
}

/**
 * Main application entry point
 * Sets up the terminal, creates application state, and runs the main event loop
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get initial keyword from command line args if provided
    let args: Vec<String> = env::args().collect();
    let initial_keyword = if args.len() > 1 { args[1].clone() } else { String::new() };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let current_dir = env::current_dir()?;
    let file_finder = FileFinder::new(current_dir.clone());
    let mut app = App::new(initial_keyword);
    
    // If there's an initial keyword, perform the initial search
    if !app.keyword.is_empty() {
        app.update_files(&file_finder);
    }

    // Add debouncing variables
    let mut last_key_time = Instant::now();
    // 20ms is short enough to feel responsive but long enough to filter duplicates
    let debounce_duration = Duration::from_millis(20);

    // Main event loop
    loop {
        terminal.draw(|f| {
            let size = f.area();
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Input area
                    Constraint::Min(1),    // File list
                    Constraint::Length(if app.show_help { 6 } else { 1 }) // Help or status bar
                ])
                .split(size);

            // Input field
            let input = Paragraph::new(app.keyword.as_str())
                .block(Block::default()
                    .title("Filter (Type to search, Esc to exit, ? for help)")
                    .borders(Borders::ALL));
            f.render_widget(input, chunks[0]);

            // File list
            let items: Vec<ListItem> = if app.files.is_empty() {
                vec![ListItem::new("No files match your filter. Type to search...")]
            } else {
                app.files
                    .iter()
                    .map(|path| {
                        let relative_path = file_finder.get_relative_path(path);
                        ListItem::new(relative_path.to_string_lossy().to_string())
                    })
                    .collect()
            };

            let list = List::new(items)
                .block(Block::default()
                    .title(format!("Files ({} matches)", app.files.len()))
                    .borders(Borders::ALL))
                .highlight_style(Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD))
                .highlight_symbol("=> ");

            let mut list_state = ListState::default();
            if !app.files.is_empty() {
                list_state.select(Some(app.selected_index));
            }
            f.render_stateful_widget(list, chunks[1], &mut list_state);

            // Help or status bar
              if app.show_help {
        let help_text = vec![
            Line::from(vec![Span::raw("↑/↓: Navigate files")]),
            Line::from(vec![Span::raw("Enter: Open selected file in VS Code")]),
            Line::from(vec![Span::raw("Esc: Exit the application")]),
            Line::from(vec![Span::raw("?: Toggle help")]),
        ];
        
        let help = Paragraph::new(help_text)
            .block(Block::default().title("Help").borders(Borders::ALL));
        f.render_widget(help, chunks[2]);
    } else {
        let status_bar = Paragraph::new("Press ? for help")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(status_bar, chunks[2]);
    }
})?;

       // Handle events with debouncing
        if let Event::Key(key) = event::read()? {
            // Only process keystrokes that are separated by at least debounce_duration
            let now = Instant::now();
            if now.duration_since(last_key_time) >= debounce_duration {
                last_key_time = now;
                
                match key.code {
                    KeyCode::Char('?') => app.toggle_help(),
                    KeyCode::Char(c) => app.add_char(c, &file_finder),
                    KeyCode::Backspace => app.remove_char(&file_finder),
                    KeyCode::Up => app.move_selection_up(),
                    KeyCode::Down => app.move_selection_down(),
                    KeyCode::Enter => {
                        if let Err(e) = app.open_selected_file(&file_finder) {
                            eprintln!("Failed to open file: {}", e);
                        }
                    },
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}