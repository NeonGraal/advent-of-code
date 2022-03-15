use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

pub struct Timing {
    start: Instant,
    dots: Instant,
    lines: Instant,
    dot: Duration,
    line: Duration,
}

impl Timing {
    pub fn start(dot_ms: u64, line_s: u64) -> Timing {
        let dot = Duration::from_millis(dot_ms);
        let line = Duration::from_secs(line_s);
        let start = Instant::now();
        let dots = Instant::now();
        let lines = Instant::now();
        Timing {
            start,
            dots,
            lines,
            dot,
            line,
        }
    }

    pub fn check<F>(&mut self, detail: F)
    where
        F: Fn() -> String,
    {
        if self.dots.elapsed() > self.dot {
            self.dots = Instant::now();
            print!(".");
            stdout().flush().unwrap();
            if self.lines.elapsed() > self.line {
                self.lines = Instant::now();
                println!(" {} {}", self.start.elapsed().as_secs(), detail());
            }
        }
    }
}
