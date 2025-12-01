use std::{error::Error, io};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use crate::models::{TodoList, Priority}; // Removed unused TodoItem

enum InputMode {
    Normal,
    Adding,             
    EditingName(u64),   
    EditingDeadline(u64), 
    Search,             
}

struct App {
    todo_list: TodoList,
    input_buffer: String,
    input_mode: InputMode,
    list_state: ListState,
    search_query: String,
    status_msg: String,   
}

impl App {
    fn new(todo_list: TodoList) -> App {
        let mut app = App {
            todo_list,
            input_buffer: String::new(),
            input_mode: InputMode::Normal,
            list_state: ListState::default(),
            search_query: String::new(),
            status_msg: String::from("Welcome! Press '?' for help."),
        };
        if !app.todo_list.items.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }

    fn get_filtered_indices(&self) -> Vec<usize> {
        self.todo_list.items.iter().enumerate()
            .filter(|(_, item)| {
                if self.search_query.is_empty() { true }
                else { item.name.to_lowercase().contains(&self.search_query.to_lowercase()) }
            })
            .map(|(i, _)| i)
            .collect()
    }

    fn up(&mut self) {
        let indices = self.get_filtered_indices();
        if indices.is_empty() { return; }

        let i = match self.list_state.selected() {
            Some(i) => {
                if let Some(pos) = indices.iter().position(|&x| x == i) {
                    if pos == 0 { indices[indices.len() - 1] } else { indices[pos - 1] }
                } else { indices[0] }
            }
            None => indices[0],
        };
        self.list_state.select(Some(i));
    }

    fn down(&mut self) {
        let indices = self.get_filtered_indices();
        if indices.is_empty() { return; }

        let i = match self.list_state.selected() {
            Some(i) => {
                 if let Some(pos) = indices.iter().position(|&x| x == i) {
                    if pos >= indices.len() - 1 { indices[0] } else { indices[pos + 1] }
                } else { indices[0] }
            }
            None => indices[0],
        };
        self.list_state.select(Some(i));
    }

    // --- FIX 1: Toggle Complete ---
    fn toggle_complete(&mut self) {
        if let Some(i) = self.list_state.selected() {
            // Scope block starts
            if let Some(item) = self.todo_list.items.get_mut(i) {
                item.completed = !item.completed;
            } 
            // Scope block ends, we are no longer borrowing the item
            
            // Now we can safely save
            self.todo_list.save().unwrap();
        }
    }

    fn delete_selected(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if let Some(item) = self.todo_list.items.get(i) {
                let id = item.id;
                self.todo_list.delete_item(id);
                self.todo_list.save().unwrap();
                self.status_msg = String::from("Task Deleted");
                self.list_state.select(None); 
            }
        }
    }

    // --- FIX 2: Cycle Priority ---
    fn cycle_priority(&mut self) {
        if let Some(i) = self.list_state.selected() {
            // 1. Modify the item
            if let Some(item) = self.todo_list.items.get_mut(i) {
                item.priority = match item.priority {
                    Priority::Low => Priority::Medium,
                    Priority::Medium => Priority::High,
                    Priority::High => Priority::Low,
                };
            }
            // 2. Reference dropped here.
            
            // 3. Save
            self.todo_list.save().unwrap();
            self.status_msg = String::from("Priority Updated");
        }
    }

    fn purge_completed(&mut self) {
        self.todo_list.delete_all_completed();
        self.todo_list.save().unwrap();
        self.status_msg = String::from("Completed Tasks Purged");
        self.list_state.select(None);
    }

    fn sort_list(&mut self) {
        self.todo_list.sort_by_priority(); 
        self.todo_list.save().unwrap();
        self.status_msg = String::from("Sorted by Priority");
    }

    fn submit_input(&mut self) {
        match self.input_mode {
            InputMode::Adding => {
                if !self.input_buffer.trim().is_empty() {
                    self.todo_list.add_item(self.input_buffer.clone(), None, None);
                    self.todo_list.save().unwrap();
                    self.status_msg = String::from("Task Added");
                }
            },
            InputMode::EditingName(id) => {
                if !self.input_buffer.trim().is_empty() {
                    self.todo_list.edit_task_name(id, self.input_buffer.clone());
                    self.todo_list.save().unwrap();
                    self.status_msg = String::from("Name Updated");
                }
            },
            InputMode::EditingDeadline(id) => {
                let val = if self.input_buffer.trim().is_empty() { None } else { Some(self.input_buffer.clone()) };
                if self.todo_list.edit_task_deadline(id, val) {
                    self.todo_list.save().unwrap();
                    self.status_msg = String::from("Deadline Updated");
                } else {
                     self.status_msg = String::from("Invalid Date Format (Use YYYY-MM-DD)");
                }
            },
            InputMode::Search => {
                self.search_query = self.input_buffer.clone();
            },
            InputMode::Normal => {}
        }
        self.input_buffer.clear();
        self.input_mode = InputMode::Normal;
    }
}

pub fn run_tui(todo_list: TodoList) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(todo_list);
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => app.down(),
                        KeyCode::Char('k') | KeyCode::Up => app.up(),
                        KeyCode::Char(' ') => app.toggle_complete(),
                        KeyCode::Char('d') => app.delete_selected(),
                        KeyCode::Char('p') => app.cycle_priority(),
                        KeyCode::Char('X') => app.purge_completed(),
                        KeyCode::Char('S') => app.sort_list(),
                        KeyCode::Char('/') => { 
                            app.input_mode = InputMode::Search;
                            app.input_buffer.clear();
                        },
                        KeyCode::Char('a') => { 
                            app.input_mode = InputMode::Adding;
                            app.input_buffer.clear();
                        },
                        KeyCode::Char('e') => { 
                             if let Some(i) = app.list_state.selected() {
                                 if let Some(item) = app.todo_list.items.get(i) {
                                     app.input_mode = InputMode::EditingName(item.id);
                                     app.input_buffer = item.name.clone(); 
                                 }
                             }
                        },
                        KeyCode::Char('D') => { 
                             if let Some(i) = app.list_state.selected() {
                                 if let Some(item) = app.todo_list.items.get(i) {
                                     app.input_mode = InputMode::EditingDeadline(item.id);
                                     if let Some(d) = item.deadline {
                                         app.input_buffer = d.to_string();
                                     } else {
                                         app.input_buffer.clear();
                                     }
                                 }
                             }
                        },
                        KeyCode::Esc => {
                            app.search_query.clear();
                            app.status_msg = String::from("Search cleared");
                        }
                        _ => {}
                    },

                    _ => match key.code {
                        KeyCode::Enter => app.submit_input(),
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                            app.input_buffer.clear();
                        },
                        KeyCode::Backspace => { app.input_buffer.pop(); },
                        KeyCode::Char(c) => { app.input_buffer.push(c); },
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), 
            Constraint::Min(1),    
            Constraint::Length(3), 
        ])
        .split(f.size());

    let title = format!(" Rusty Todo TUI | Filter: '{}' | Msg: {} ", app.search_query, app.status_msg);
    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = app.todo_list.items.iter().enumerate()
        .filter(|(_, item)| {
             if app.search_query.is_empty() { true }
             else { item.name.to_lowercase().contains(&app.search_query.to_lowercase()) }
        })
        .map(|(original_index, item)| {
            let status = if item.completed { "[X]" } else { "[ ]" };
            let priority_icon = match item.priority {
                Priority::High => "!!!",
                Priority::Medium => " !!",
                Priority::Low => "  !",
            };
            let color = match item.priority {
                Priority::High => Color::Red,
                Priority::Medium => Color::Yellow,
                Priority::Low => Color::Green,
            };
            let deadline = match item.deadline {
                Some(d) => d.to_string(),
                None => String::from("          "),
            };
            
            let style = if Some(original_index) == app.list_state.selected() {
                 Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                 Style::default().fg(color)
            };

            let line = format!("{} {} | {} | {} | {}", status, item.id, priority_icon, deadline, item.name);
            ListItem::new(line).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Tasks ").borders(Borders::ALL));
    
    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    let input_title = match app.input_mode {
        InputMode::Normal => " Normal Mode ",
        InputMode::Adding => " Add Task ",
        InputMode::EditingName(_) => " Edit Name ",
        InputMode::EditingDeadline(_) => " Edit Deadline (YYYY-MM-DD) ",
        InputMode::Search => " Search ",
    };

    let footer_text = match app.input_mode {
        InputMode::Normal => String::from(" (a)Add (e)Name (p)Prio (D)Date (space)Done (X)Purge (/)Search (S)Sort (q)Quit "),
        _ => app.input_buffer.clone(),
    };

    let footer_color = match app.input_mode {
        InputMode::Normal => Color::White,
        _ => Color::Yellow,
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(footer_color))
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(footer, chunks[2]);

    match app.input_mode {
        InputMode::Normal => {},
        _ => {
             f.set_cursor(
                chunks[2].x + 1 + app.input_buffer.len() as u16,
                chunks[2].y + 1,
            )
        }
    }
}
