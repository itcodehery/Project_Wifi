use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal,
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::process::Command;

#[derive(Debug, Default, Clone)]
struct WifiInfo {
    ssid: String,
    network_type: String,
    authentication: String,
    encryption: String,
}

#[derive(Debug, Default)]
struct AppState {
    wifi_list: Vec<WifiInfo>,
    l_state: ListState,
}

impl AppState {
    fn new(wifi_list: Vec<WifiInfo>) -> AppState {
        AppState {
            wifi_list,
            l_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn next(&mut self) {
        let i = match self.l_state.selected() {
            Some(i) => {
                if i >= self.wifi_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.l_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.l_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.wifi_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.l_state.select(Some(i));
    }
}

fn main() -> Result<()> {
    let wifi_list = get_wifi_networks()?;
    let mut state = AppState::new(wifi_list);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => break,
                event::KeyCode::Char('j') | event::KeyCode::Down => state.next(),
                event::KeyCode::Char('k') | event::KeyCode::Up => state.previous(),
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, state: &mut AppState) {
    let areas = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let list_items: Vec<ListItem> = state
        .wifi_list
        .iter()
        .map(|w| ListItem::new(w.ssid.clone()))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title("WiFi Networks")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, areas[0], &mut state.l_state);

    if let Some(selected) = state.l_state.selected() {
        if let Some(wifi) = state.wifi_list.get(selected) {
            let info = format!(
                "SSID: {}\nNetwork Type: {}\nAuthentication: {}\nEncryption: {}\n",
                wifi.ssid, wifi.network_type, wifi.authentication, wifi.encryption
            );
            let paragraph = Paragraph::new(info)
                .block(Block::default().title("Information").borders(Borders::ALL));
            frame.render_widget(paragraph, areas[1]);
        }
    }
}

fn get_wifi_networks() -> Result<Vec<WifiInfo>> {
    let output = Command::new("netsh")
        .args(["wlan", "show", "networks"])
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut wifi_list = Vec::new();
    let mut current_wifi = WifiInfo::default();
    let mut in_network_block = false;

    for line in output_str.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("SSID") {
            if in_network_block {
                wifi_list.push(current_wifi.clone());
            }
            current_wifi = WifiInfo::default();
            if let Some(ssid) = trimmed_line.split(':').nth(1) {
                current_wifi.ssid = ssid.trim().to_string();
            }
            in_network_block = true;
        } else if in_network_block {
            if trimmed_line.starts_with("Network type") {
                if let Some(ntype) = trimmed_line.split(':').nth(1) {
                    current_wifi.network_type = ntype.trim().to_string();
                }
            } else if trimmed_line.starts_with("Authentication") {
                if let Some(auth) = trimmed_line.split(':').nth(1) {
                    current_wifi.authentication = auth.trim().to_string();
                }
            } else if trimmed_line.starts_with("Encryption") {
                if let Some(enc) = trimmed_line.split(':').nth(1) {
                    current_wifi.encryption = enc.trim().to_string();
                }
            }
        }
    }
    if in_network_block {
        wifi_list.push(current_wifi);
    }

    Ok(wifi_list)
}
