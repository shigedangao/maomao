use termion::color;

#[derive(Debug)]
pub struct Logger {
    quiet: bool
}

pub enum LogLevel<'a> {
    Info(&'a str),
    Warning(&'a str),
    Success(&'a str),
}

impl Logger {
    pub fn new(quiet: bool) -> Logger {
        Logger { quiet }
    } 

    pub fn print(&self, level: LogLevel) {
        if self.quiet {
           return;
        }

        match level {
            LogLevel::Info(msg) => println!("{}", msg),
            LogLevel::Success(msg) => println!("{}{}", color::Fg(color::LightBlue), msg),
            LogLevel::Warning(msg) => println!("{}{}", color::Fg(color::Yellow), msg)
        }
    } 
}