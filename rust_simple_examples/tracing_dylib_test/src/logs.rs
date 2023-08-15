use tracing::Level;

#[derive(Debug)]
pub struct LogObj {
    pub log_max_level: Level,
    pub span_name: String,
}

impl LogObj {
    pub fn new(log_max_level: Option<Level>, span_name: Option<String>) -> Self {
        let log_max_level = match log_max_level {
            Some(max_level) => max_level,
            None => Level::INFO,
        };

        let span_name = match span_name {
            Some(name) => name,
            None => String::new(),
        };

        LogObj {
            log_max_level,
            span_name,
        }
    }
}
