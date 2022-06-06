use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn print_elapsed_time(&self) {
        println!("\n\nProgram executed in {:?}", self.start.elapsed());
    }
}
