# Ashdoro
A simple Pomodoro timer CLI written in Rust.

## Usage
```bash
ashdoro --work 25 --rest 5 --sessions 4
```
If no arguments are provided, values from `ashdoro.toml` are used, falling back to defaults.

Default values: Work: 25, Rest: 5, Sessions: 3

## Options
| Flag         | Description                               | Default |
|--------------|--------------------------------------------|---------|
| `--work`     | Work session length (min)                 | 25      |
| `--rest`     | Rest session length (min)                 | 5       |
| `--sessions` | Number of work sessions                   | 3       |
| `--config`   | Prints the current config file contents   | false   |

Run `ashdoro --help` for full flag descriptions.

## Build
```bash
cargo build --release
```

## Config
Create an `ashdoro.toml` file in your OS config directory:

| OS      | Path                                |
|---------|--------------------------------------|
| Linux   | `$HOME/.config/ashdoro.toml`         |
| macOS   | `$HOME/Library/Application Support/ashdoro.toml` |
| Windows | `%APPDATA%\ashdoro.toml`             |

Example:
```toml
work = 30
rest = 10
sessions = 4
```
CLI flags always override config file values.
