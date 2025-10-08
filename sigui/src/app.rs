use crate::wifi::{get_connected_ssid, get_wifi_networks, WifiInfo};
use color_eyre::eyre::Result;
use ratatui::widgets::ListState;

#[derive(Debug, Default)]
pub struct AppState {
    pub wifi_list: Vec<WifiInfo>,
    pub l_state: ListState,
    pub connected_ssid: Option<String>,
    pub show_password_popup: bool,
    pub password_input: String,
    pub connecting_to_ssid: Option<String>,
    pub speed_test_result: Option<String>,
    pub is_testing_speed: bool,
}

impl AppState {
    pub fn new(wifi_list: Vec<WifiInfo>) -> AppState {
        AppState {
            wifi_list,
            l_state: ListState::default().with_selected(Some(0)),
            connected_ssid: get_connected_ssid().unwrap_or(None),
            show_password_popup: false,
            password_input: String::new(),
            connecting_to_ssid: None,
            speed_test_result: None,
            is_testing_speed: false,
        }
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.wifi_list = get_wifi_networks()?;
        self.connected_ssid = get_connected_ssid()?;
        Ok(())
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
