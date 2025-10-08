use crate::{
    app::AppState,
    ui::render,
    wifi::{connect_with_password, run_speed_test},
};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::process::Command;

pub fn run(mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                if state.show_password_popup {
                    match key.code {
                        event::KeyCode::Enter => {
                            if let Some(ssid) = state.connecting_to_ssid.take() {
                                if let Err(e) = connect_with_password(&ssid, &state.password_input)
                                {
                                    // Handle error, maybe show a message to the user
                                    eprintln!("Failed to connect: {}", e);
                                }
                            }
                            state.show_password_popup = false;
                            state.password_input.clear();
                            state.refresh()?;
                        }
                        event::KeyCode::Char(c) => {
                            state.password_input.push(c);
                        }
                        event::KeyCode::Backspace => {
                            state.password_input.pop();
                        }
                        event::KeyCode::Esc => {
                            state.show_password_popup = false;
                            state.password_input.clear();
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        event::KeyCode::Esc | event::KeyCode::Char('q') => break,
                        event::KeyCode::Char('j') | event::KeyCode::Down => state.next(),
                        event::KeyCode::Char('k') | event::KeyCode::Up => state.previous(),
                        event::KeyCode::Enter => {
                            if let Some(selected) = state.l_state.selected() {
                                if let Some(wifi) = state.wifi_list.get(selected) {
                                    if wifi.authentication != "Open" {
                                        state.show_password_popup = true;
                                        state.connecting_to_ssid = Some(wifi.ssid.clone());
                                    } else if let Some(connected_ssid) = &state.connected_ssid {
                                        if wifi.ssid == *connected_ssid {
                                            Command::new("netsh")
                                                .args(["wlan", "disconnect"])
                                                .output()?;
                                        } else {
                                            Command::new("netsh")
                                                .args([
                                                    "wlan",
                                                    "connect",
                                                    &format!("name={}", wifi.ssid),
                                                    &format!("ssid={}", wifi.ssid),
                                                    "interface=Wi-Fi",
                                                ])
                                                .output()?;
                                        }
                                    } else {
                                        Command::new("netsh")
                                            .args([
                                                "wlan",
                                                "connect",
                                                &format!("name={}", wifi.ssid),
                                                &format!("ssid={}", wifi.ssid),
                                                "interface=Wi-Fi",
                                            ])
                                            .output()?;
                                    }
                                }
                            }
                        }
                        event::KeyCode::Char('s') => {
                            state.is_testing_speed = true;
                            terminal.draw(|frame| render(frame, state))?;

                            match run_speed_test() {
                                Ok(speed) => state.speed_test_result = Some(speed),
                                Err(e) => state.speed_test_result = Some(format!("Err: {}", e)),
                            }
                            state.is_testing_speed = false;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
