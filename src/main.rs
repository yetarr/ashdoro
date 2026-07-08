use std::{thread::sleep, time::Duration};

use clap::Parser;

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

enum Phase {
    Work,
    Rest,
}

struct Pomodoro {
    phase: Phase,
    time: u32,
    streak: u8,
    settings: Args,
}

impl Pomodoro {
    fn start(settings: Args) -> Pomodoro {
        Pomodoro {
            phase: Phase::Work,
            time: settings.work,
            streak: 0,
            settings,
        }
    }

    fn decrement(&mut self) -> u32 {
        self.time -= 1;
        self.time
    }

    fn next(&mut self) -> bool {
        match self.phase {
            Phase::Work => {
                self.phase = Phase::Rest;
                self.time = self.settings.rest;
                self.streak += 1;

                if self.streak == self.settings.sessions {
                    return true;
                }
            }

            Phase::Rest => {
                self.phase = Phase::Work;
                self.time = self.settings.work;
            }
        }

        false
    }

    fn session(&self) -> &str {
        match self.phase {
            Phase::Work => "Work",
            Phase::Rest => "Rest",
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut pomodoro = Pomodoro::start(args);
    let mut time = pomodoro.time;

    loop {
        println!("{}", time);
        time = pomodoro.decrement();
        
        if time == 0 {
            println!("{} session done!", pomodoro.session());
            if pomodoro.next() { break; }
        }

        sleep(Duration::from_secs(1));
    }

    println!("Session done! One more productive day completed, nice work! (:");
}
