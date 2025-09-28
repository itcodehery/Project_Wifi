use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Stylize},
    widgets::{Block, List, ListItem},
};

#[derive(Debug, Default)]
struct AppState {
    todo_list: Vec<TodoItem>,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.todo_list.push(TodoItem {
        is_done: false,
        description: String::from("Learn Ratatui"),
    });
    state.todo_list.push(TodoItem {
        is_done: false,
        description: String::from("Mark the Attendance"),
    });

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|frame| render(frame, state))?;
        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_border] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    frame.render_widget(
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .fg(Color::Red),
        frame.area(),
    );

    let list = List::new(
        state
            .todo_list
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    );

    frame.render_widget(list, inner_border);
}
