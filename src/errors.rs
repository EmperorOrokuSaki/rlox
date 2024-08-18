#[derive(Debug)]
pub enum RuntimeError {
    ParseError
}

pub fn rlox_error(line: u64, message: &str) {
    report(line, "", message);
}

pub fn report(line: u64, location: &str, message: &str) {
    println!("[Line {}] Error {}: {}", line, location, message);
}
