pub struct LoggerState {
    is_verbose: bool,
}

impl LoggerState {
    pub fn new(is_verbose: bool) -> Self {
        Self { is_verbose }
    }
    pub fn set_verbose(&mut self, is_verbose: bool) {
        self.is_verbose = is_verbose;
    }
    pub fn log(&self, message: &str) {
        if self.is_verbose {
            println!("{}", message)
        }
    }
}
