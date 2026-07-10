use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread::sleep, time::Duration};
use std::path::Path;

use clap::Parser;
use notify_rust::{Notification, Timeout};

mod pomodoro;
mod renderer;

use crate::pomodoro::{Pomodoro, Settings};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    work: Option<u32>,

    #[arg(long)]
    rest: Option<u32>,

    #[arg(long)]
    sessions: Option<u8>,
}

#[derive(serde::Deserialize, Default)]
struct Config {
    work: Option<u32>,
    rest: Option<u32>,
    sessions: Option<u8>,
}

fn load_config(path: &Path) -> Config {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn notify(content: String) {
    if let Err(e) = Notification::new()
        .summary("Ashdoro")
        .body(&content)
        .icon("kitty")
        .timeout(Timeout::Milliseconds(5000))
        .show()
    {
        eprintln!("notification failed: {}", e);
    }
}

fn main() {
    let config_path = dirs::config_dir().unwrap().join("ashdoro.toml");
    
    let args = Args::parse();
    let config = load_config(config_path.as_path());
    
    let work = args.work.or(config.work).unwrap_or(25);
    let rest = args.rest.or(config.rest).unwrap_or(5);
    let sessions = args.sessions.or(config.sessions).unwrap_or(3);
    
    let mut pomodoro = Pomodoro::start(
        Settings::new(work, rest, sessions)
    );

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        renderer::tick(pomodoro.remaining_time());
        sleep(Duration::from_secs(1));
        
        pomodoro.decrement();
        
        if pomodoro.is_done() {
            renderer::phase_done(pomodoro.phase());
            notify(format!("{} session done!", pomodoro.phase()));
            
            if !pomodoro.next() {
                renderer::session_done(pomodoro.sessions());
                notify(format!("Session completed with {} cycles. Nice Work!", pomodoro.sessions()));
                return;
            }
            
            sleep(Duration::from_secs(1));
        }
    }

    renderer::session_interrupted(pomodoro.streak());
}
