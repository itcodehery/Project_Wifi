# SiGUI

A Terminal User Interface (TUI) for managing Wi-Fi connections.

## About

SiGUI is a lightweight and efficient command-line tool for managing wireless networks. It provides a simple and intuitive terminal-based interface for scanning, connecting to, and managing Wi-Fi networks. The application is built with Rust and uses the `ratatui` library to create the terminal user interface.

This project aims to offer a user-friendly alternative to graphical network managers, ideal for developers, system administrators, and anyone who prefers working in the terminal.

## Features

*   **Scan for Networks:** Scan for available Wi-Fi networks and display them in a list.
*   **Connect to Networks:** Connect to open or password-protected (WPA/WPA2/WPA3) networks.
*   **Disconnect:** Disconnect from the currently connected network.
*   **Network Information:** View detailed information about the current connection, such as SSID, signal strength, and IP address.
*   **Manage Saved Networks:** View and manage a list of saved Wi-Fi profiles.

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

Launch the application by running the executable. The main screen will display a list of available Wi-Fi networks.

### Keybindings

*   **Up/Down Arrows:** Navigate through the list of networks.
*   **Enter:** Connect to the selected network. You will be prompted to enter a password if required.
*   **d:** Disconnect from the current network.
*   **i:** View detailed information about the selected network.
*   **m:** View and manage saved network profiles.
*   **q:** Quit the application.

## Development

SiGUI is currently in active development. Contributions are welcome! If you would like to contribute, please fork the repository and submit a pull request.

## License

This project is not yet licensed.
