use std::io::{self, Write};

use crate::pomodoro::Phase;

pub fn format_time(secs: u32) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

pub fn tick(time: u32) {
    print!("\r\x1B[K{}", format_time(time));
    io::stdout().flush().unwrap();
}

pub fn phase_done(phase: &Phase) {
    print!("\r\x1B[K{} session done!", phase);
    io::stdout().flush().unwrap();
}

pub fn session_done(cycles: u8) {
    println!("\nSession completed with {} cycles. Nice Work!", cycles);
}

pub fn session_interrupted(streak: u8) {
    println!("\nPomodoro interrupted with {} cycles completed! ", streak);
}
