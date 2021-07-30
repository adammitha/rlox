pub struct Error {
    pub had_error: bool,
}

impl Error {
    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
