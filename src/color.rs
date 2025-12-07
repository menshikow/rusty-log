use colored::*;
use once_cell::sync::Lazy;
use regex::Regex;

static ERROR_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\berror\b").unwrap(),
        Regex::new(r"(?i)\bfatal\b").unwrap(),
        Regex::new(r"(?i)\bfailed\b").unwrap(),
        Regex::new(r"(?i)\bexception\b").unwrap(),
    ]
});

static WARN_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bwarn\b").unwrap(),
        Regex::new(r"(?i)\bwarning\b").unwrap(),
        Regex::new(r"(?i)\bcaution\b").unwrap(),
    ]
});

static INFO_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\binfo\b").unwrap(),
        Regex::new(r"(?i)\binformation\b").unwrap(),
    ]
});

static DEBUG_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"(?i)\bdebug\b").unwrap(),
        Regex::new(r"(?i)\btrace\b").unwrap(),
    ]
});

pub struct Colorizer {
    enabled: bool,
    highlight: bool,
}

impl Colorizer {
    pub fn new(enabled: bool, highlight: bool) -> Self {
        Self { enabled, highlight }
    }

    pub fn colorize(&self, line: &str, matches: &[(usize, usize)]) -> String {
        if !self.enabled {
            return line.to_string();
        }

        // First, determine the base color based on log level
        let level = self.detect_log_level(line);

        // If we have matches to highlight, do that
        if self.highlight && !matches.is_empty() {
            self.highlight_matches(line, matches, level)
        } else {
            self.colorize_line(line, level)
        }
    }

    fn detect_log_level(&self, line: &str) -> LogLevel {
        for pattern in ERROR_PATTERNS.iter() {
            if pattern.is_match(line) {
                return LogLevel::Error;
            }
        }

        for pattern in WARN_PATTERNS.iter() {
            if pattern.is_match(line) {
                return LogLevel::Warn;
            }
        }

        for pattern in INFO_PATTERNS.iter() {
            if pattern.is_match(line) {
                return LogLevel::Info;
            }
        }

        for pattern in DEBUG_PATTERNS.iter() {
            if pattern.is_match(line) {
                return LogLevel::Debug;
            }
        }

        LogLevel::None
    }

    fn colorize_line(&self, line: &str, level: LogLevel) -> String {
        if !self.enabled {
            return line.to_string();
        }

        match level {
            LogLevel::Error => line.red().bold().to_string(),
            LogLevel::Warn => line.yellow().to_string(),
            LogLevel::Info => line.blue().to_string(),
            LogLevel::Debug => line.cyan().to_string(),
            LogLevel::None => line.to_string(),
        }
    }

    fn highlight_matches(
        &self,
        line: &str,
        matches: &[(usize, usize)],
        level: LogLevel,
    ) -> String {
        if matches.is_empty() {
            return self.colorize_line(line, level);
        }

        let mut result = String::new();
        let mut last_end = 0;

        for &(start, end) in matches {
            // Add text before match
            if start > last_end {
                let before = &line[last_end..start];
                result.push_str(&self.colorize_line(before, level));
            }

            // Add highlighted match
            let matched = &line[start..end];
            result.push_str(&matched.on_yellow().black().to_string());

            last_end = end;
        }

        // Add remaining text
        if last_end < line.len() {
            let remaining = &line[last_end..];
            result.push_str(&self.colorize_line(remaining, level));
        }

        result
    }
}

#[derive(Clone, Copy)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    None,
}
