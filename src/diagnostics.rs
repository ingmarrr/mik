use crate::token::SourceLocation;

pub struct Diagnostic<'a> {
    pub level: Level,
    pub message: &'a str,
    pub location: SourceLocation<'a>,
}

pub enum Level {
    Error,
    Warning,
    Info,
}

pub trait DiagnosticEmitter {
    fn emit(&mut self, diagnostic: Diagnostic);
}

pub struct DefaultEmitter;

impl DiagnosticEmitter for DefaultEmitter {
    fn emit(&mut self, diagnostic: Diagnostic) {
        let location = diagnostic.location;
        let file = location.file.unwrap_or("unknown file");
        match diagnostic.level {
            Level::Error => eprintln!(
                "[ERROR]: {}:{}:{}  --> {}",
                file, location.row, location.col, diagnostic.message
            ),
            Level::Warning => eprintln!(
                "[WARN]: {}:{}:{}  --> {}",
                file, location.row, location.col, diagnostic.message
            ),
            Level::Info => eprintln!(
                "[INFO]: {}:{}:{}  --> {}",
                file, location.row, location.col, diagnostic.message
            ),
        }
    }
}
