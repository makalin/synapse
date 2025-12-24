// use gpui::*; // Commented out for CLI version
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl LogLevel {
    pub fn color_code(&self) -> u32 {
        match self {
            LogLevel::Trace => 0x666666,
            LogLevel::Debug => 0x8888ff,
            LogLevel::Info => 0x00ff00,
            LogLevel::Warn => 0xffff00,
            LogLevel::Error => 0xff6666,
            LogLevel::Critical => 0xff0000,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        }
    }
}

pub struct LogPanel {
    logs: VecDeque<LogEntry>,
    filter: LogFilter,
    max_logs: usize,
    auto_scroll: bool,
}

#[derive(Debug, Clone)]
pub struct LogFilter {
    pub level: Option<LogLevel>,
    pub source: Option<String>,
    pub search_query: String,
}

impl LogPanel {
    pub fn new(_cx: &mut ()) -> Self {
        Self {
            logs: VecDeque::new(),
            filter: LogFilter {
                level: None,
                source: None,
                search_query: String::new(),
            },
            max_logs: 10000,
            auto_scroll: true,
        }
    }

    pub fn add_log(&mut self, entry: LogEntry) {
        self.logs.push_back(entry);
        if self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }
    }

    pub fn log(&mut self, level: LogLevel, source: &str, message: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.add_log(
            LogEntry {
                timestamp,
                level,
                source: source.to_string(),
                message,
                metadata: None,
            },
        );
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }

    pub fn filtered_logs(&self) -> Vec<&LogEntry> {
        self.logs
            .iter()
            .filter(|log| {
                if let Some(ref level) = self.filter.level {
                    if log.level != *level {
                        return false;
                    }
                }
                if let Some(ref source) = self.filter.source {
                    if !log.source.contains(source) {
                        return false;
                    }
                }
                if !self.filter.search_query.is_empty() {
                    let query = self.filter.search_query.to_lowercase();
                    if !log.message.to_lowercase().contains(&query)
                        && !log.source.to_lowercase().contains(&query)
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn format_timestamp(&self, timestamp: u64) -> String {
        // Simple timestamp formatting
        let datetime = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp);
        format!("{:?}", datetime)
            .chars()
            .take(19)
            .collect()
    }

    pub fn get_logs(&self) -> &VecDeque<LogEntry> {
        &self.logs
    }
}

// UI rendering code commented out for CLI version
