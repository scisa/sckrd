pub struct Counter {
    counter: usize,
}

impl Counter {
    pub fn new() -> Self {
        return Self { counter: 0 };
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn print_result(&self) {
        println!("\n\nResult Count: {}", self.counter);
    }
}
