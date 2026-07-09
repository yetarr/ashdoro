use std::fmt;

pub struct Settings {
    work_time: u32,
    rest_time: u32,
    sessions: u8,
}

impl Settings {
    pub fn new(work: u32, rest: u32, sessions: u8) -> Settings {
        Settings {
            work_time: work,
            rest_time: rest,
            sessions,
        }
    }
}

pub enum Phase {
    Work,
    Rest,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Phase::Work => "Work",
            Phase::Rest => "Rest",
        })
    }
}

pub struct Pomodoro {
    phase: Phase,
    time: u32,
    streak: u8,
    settings: Settings,
}

impl Pomodoro {
    pub fn start(settings: Settings) -> Pomodoro {
        Pomodoro {
            phase: Phase::Work,
            time: settings.work_time * 60,
            streak: 0,
            settings,
        }
    }
    
    pub fn remaining_time(&self) -> u32 {
        self.time
    }

    pub fn streak(&self) -> u8 {
        self.streak
    }

    pub fn sessions(&self) -> u8 {
        self.settings.sessions
    }

    pub fn phase(&self) -> &Phase {
        &self.phase
    }
}

impl Pomodoro {
    pub fn decrement(&mut self) -> u32 {
        self.time -= 1;
        self.time
    }

    pub fn next(&mut self) -> bool {
        match self.phase {
            Phase::Work => {
                self.phase = Phase::Rest;
                self.time = self.settings.rest_time * 60;
                self.streak += 1;

                if self.streak == self.settings.sessions {
                    return false;
                }
            }

            Phase::Rest => {
                self.phase = Phase::Work;
                self.time = self.settings.work_time * 60;
            }
        }

        true
    }

    pub fn is_done(&self) -> bool {
        self.time == 0
    }
}
