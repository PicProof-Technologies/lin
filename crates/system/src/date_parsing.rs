use chrono::NaiveDate;
use core::fmt;
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DateFormat {
    US,             // MM/DD/YYYY
    UK,             // DD/MM/YYYY
    ISO,            // YYYY-MM-DD
    Custom(String), //  custom date formats
}

trait DateParser {
    fn parse_date(&self, date_str: &str) -> Result<NaiveDate, DateParseError>;
}
impl DateConfig {
    fn new() -> Self {
        let mut formats = HashMap::new();
        formats.insert(DateFormat::US, "%m/%d/%Y".to_string());
        formats.insert(DateFormat::UK, "%d/%m/%Y".to_string());
        formats.insert(DateFormat::ISO, "%Y-%m-%d".to_string());
        Self {
            formats,
            default_format: None,
        }
    }

    fn add_custom_format(&mut self, format: DateFormat, pattern: String) {
        self.formats.insert(format, pattern);
    }
    fn set_default_format(&mut self, pattern: String) {
        self.default_format = Some(pattern);
    }
}
struct DateConfig {
    formats: HashMap<DateFormat, String>,
    default_format: Option<String>,
}
struct DateParserImpl {
    config: DateConfig,
}
impl DateParserImpl {
    fn new(config: DateConfig) -> Self {
        Self { config }
    }
}

impl DateParser for DateParserImpl {
    fn parse_date(&self, date_str: &str) -> Result<NaiveDate, DateParseError> {
        let mut last_error = None;

        for (format, pattern) in &self.config.formats {
            match NaiveDate::parse_from_str(date_str, pattern) {
                Ok(date) => return Ok(date),
                Err(_) => last_error = Some(DateParseError::InvalidFormat(date_str.to_string())),
            }
        }

        if let Some(ref default_pattern) = self.config.default_format {
            NaiveDate::parse_from_str(date_str, default_pattern)
                .map_err(|_| DateParseError::ParseError("Failed with default format".to_string()))
        } else {
            Err(last_error.unwrap_or(DateParseError::IncompleteData))
        }
    }
}
#[derive(Debug)]
enum DateParseError {
    InvalidFormat(String),
    IncompleteData,
    ParseError(String),
}
impl fmt::Display for DateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DateParseError::InvalidFormat(ref msg) => write!(f, "Invalid date format: {}", msg),
            DateParseError::IncompleteData => write!(f, "Incomplete data for parsing date"),
            DateParseError::ParseError(ref msg) => write!(f, "Error parsing date: {}", msg),
        }
    }
}
impl std::error::Error for DateParseError {}
