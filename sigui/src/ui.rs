use crate::app::AppState;
use ratatui::
{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let main_layout = Layout::vertical([
        Constraint::Length(1), // Title bar
        Constraint::Min(0),    // Main content
        Constraint::Length(1), // Bottom bar
    ])
    .split(frame.area());

    let title = Paragraph::new("sigui v0.0.1")
        .style(Color::LightRed)
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(title, main_layout[0]);

    let areas = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    let list_items: Vec<ListItem> = state
        .wifi_list
        .iter()
        .map(|w| {
            let mut ssid = w.ssid.clone();
            if let Some(connected_ssid) = &state.connected_ssid {
                if w.ssid == *connected_ssid {
                    ssid = format!("* {} (Connected)", ssid);
                }
            }
            ListItem::new(ssid)
        })
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .title("WiFi Networks")
                .title_style(Color::Yellow)
                .borders(Borders::ALL)
                .border_style(Color::LightGreen),
        )
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Green),
        );

    frame.render_stateful_widget(list, areas[0], &mut state.l_state);

    let right_side_layout = Layout::vertical([
        Constraint::Percentage(50), // Information
        Constraint::Percentage(50), // Speed Test
    ])
    .split(areas[1]);

    let info_area = right_side_layout[0];
    let speed_test_area = right_side_layout[1];

    if let Some(selected) = state.l_state.selected() {
        if let Some(wifi) = state.wifi_list.get(selected) {
            let info = format!(
                "SSID: {}\nNetwork Type: {}\nAuthentication: {}\nEncryption: {}\n",
                wifi.ssid,
                wifi.network_type,
                wifi.authentication,
                wifi.encryption
            );
            let paragraph = Paragraph::new(info).block(
                Block::default()
                    .title("Information")
                    .borders(Borders::ALL)
                    .title_style(Color::Yellow),
            );
            frame.render_widget(paragraph, info_area);
        }
    }

    let speed_test_text = if state.is_testing_speed {
        "Testing speed...".to_string()
    } else {
        state
            .speed_test_result
            .clone()
            .unwrap_or_else(|| "Press 's' to start".to_string())
    };
    let speed_test_paragraph = Paragraph::new(speed_test_text).block(
        Block::default()
            .title("Speed Test")
            .borders(Borders::ALL)
            .title_style(Color::Yellow),
    );
    frame.render_widget(speed_test_paragraph, speed_test_area);

    let help_text = if state.is_testing_speed {
        "Testing speed...".to_string()
    } else {
        "Esc/q: quit | j/k/up/down: navigate | Enter: connect/disconnect | s: speed test"
            .to_string()
    };

    let help_paragraph = Paragraph::new(help_text).block(Block::default());
    frame.render_widget(help_paragraph, main_layout[2]);

    if state.show_password_popup {
        let area = frame.area();
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Min(3), // Height for the popup
                Constraint::Percentage(40),
            ])
            .split(area);

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Min(30), // Width for the popup
                Constraint::Percentage(40),
            ])
            .split(popup_layout[1])[1];

        let popup_text = format!(
            "Enter password for {}:\n{}",
            state.connecting_to_ssid.as_deref().unwrap_or(""),
            state
                .password_input
                .chars()
                .map(|_| '*')
                .collect::<String>()
        );
        let popup = Paragraph::new(popup_text)
            .block(Block::default().title("Password").borders(Borders::ALL));

        frame.render_widget(Clear, popup_area); //this clears the background
        frame.render_widget(popup, popup_area);
    }
}
