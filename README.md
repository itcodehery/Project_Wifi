

# SiGUI
![<ALT TEXT>](https://img.shields.io/badge/version-v0.0.1ALPHA-<COLOR>)
[![downloads](https://img.shields.io/github/downloads/itcodehery/Project_Wifi/total?color=brightgreen)](https://github.com/itcodehery/Project_Wifi/releases)

A Terminal User Interface (TUI) for managing Wi-Fi connections on Windows.

## About

SiGUI is a lightweight and efficient command-line tool for managing wireless networks. It provides a simple and intuitive terminal-based interface for scanning, connecting to, and managing Wi-Fi networks. The application is built with Rust, uses the `ratatui` library to create the terminal user interface, and currently works on Windows by using `netsh`.

This project aims to offer a user-friendly alternative to graphical network managers, ideal for developers, system administrators, and anyone who prefers working in the terminal.

## Features

*   **Scan for Networks:** Scan for available Wi-Fi networks and display them in a list.
*   **Connect to Networks:** Connect to open or password-protected (WPA2-PSK) networks.
*   **Disconnect:** Disconnect from the currently connected network.
*   **Network Information:** View detailed information about the selected network, such as SSID, network type, and authentication.
*   **Speed Test:** Run a basic speed test to check the current connection's download speed.

## Installation

This project is built with Rust. You will need to have the Rust toolchain (including `cargo`) installed.

1.  Clone the repository:
    ```sh
    git clone <repository-url>
    ```
2.  Navigate to the project directory:
    ```sh
    cd sigui
    ```
3.  Build and run the application in release mode:
    ```sh
    cargo run --release
    ```

## Usage

Launch the application by running the executable. The main screen will display a list of available Wi-Fi networks and their information.

### Keybindings

*   **Up/Down Arrows** or **k/j**: Navigate through the list of networks.
*   **Enter**: Connect to the selected network. If the network is password-protected, you will be prompted to enter it. If you are already connected to the selected network, this will disconnect you.
*   **s**: Run a speed test for the current connection.
*   **q** or **Esc**: Quit the application.

## Development

SiGUI is currently in active development. Contributions are welcome! If you would like to contribute, please fork the repository and submit a pull request.

## License

This project is not yet licensed.
