# Heimdall

## Note: This project is a WIP, and the following docs may be uncomplete.

## Features:

- An extensible hotkey daemon which supports multiple modifiers, and custom commands.

## Installation
To get started, simply clone the whole repository, and build it with Rust:

    $ git clone https://github.com/uzaaft/idun.git
    $ cd heimdall
    $ cargo run --release


## Getting started
To get started with Idun, follow these steps:

1. Create a config file with the path `$XDG_CONFIG_HOME/heimdall/config.toml`
    The config file has the following format:
    ```toml
   [[bindings]]
    key = "C"
    modifiers = ["Ctrl", "Shift"]
    command = "echo hello"
    [[bindings]]
    key = "D"
    modifiers = ["Ctrl"]
    command = "osascript -e 'display notification  with title \"Hello 👋!\" subtitle \"Hello from Heimdall 😊\" sound name \"Crystal\"'"
```
2. Start heimdall with the `cargo run` command
