pub struct SimpleErrorHandler {
    pub had_error: bool,
}

impl SimpleErrorHandler {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
        self.had_error = true;
    }
}
