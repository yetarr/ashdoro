# Ashdoro

A simple Pomodoro timer CLI written in Rust.

## Usage

```bash
cargo run -- --work 25 --rest 5 --sessions 4
```

## Options

| Flag         | Description                               | Default |
|--------------|-------------------------------------------|---------|
| `--work`     | Work session length (min)                 | 25      |
| `--rest`     | Rest session length (min)                 | 5       |
| `--sessions` | Number of work sessions                   | 3       |
| `--sessions` | Prints the current config file contents   | false   |

## Build

```bash
cargo build --release
```

## Config

Create a ashdoro.toml file on your OS config directory. 

| OS           | Path                                          |
|--------------|-----------------------------------------------|
| Linux        | $HOME/.config                                 |
| macOS        | $HOME/Library/Application Support             |
| Linux        | $HOME\AppData\Roaming                         |
