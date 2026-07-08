# Pomodoro

A simple Pomodoro timer CLI written in Rust.

## Usage

```bash
cargo run -- --work 25 --rest 5 --sessions 4
```

## Options

| Flag         | Description              | Default |
|--------------|---------------------------|---------|
| `--work`     | Work session length (min) | 25      |
| `--rest`     | Rest session length (min) | 5       |
| `--sessions` | Number of work sessions   | 3       |

## Build

```bash
cargo build --release
```