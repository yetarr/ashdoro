use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{thread::sleep, time::Duration};

use clap::Parser;

mod pomodoro;
mod renderer;

use crate::pomodoro::{Pomodoro, Settings};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 25)]
    work: u32,

    #[arg(long, default_value_t = 5)]
    rest: u32,

    #[arg(long, default_value_t = 3)]
    sessions: u8,
}

fn main() {
    let args = Args::parse();
    let mut pomodoro = Pomodoro::start(
        Settings::new(args.work, args.rest, args.sessions)
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
            if !pomodoro.next() {
                renderer::session_done(pomodoro.sessions());
                return;
            }
            
            sleep(Duration::from_secs(1));
        }
    }

    renderer::session_interrupted(pomodoro.streak());
}
