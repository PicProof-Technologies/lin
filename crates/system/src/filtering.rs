use regex::Regex;
use std::fmt;
use std::{collections::HashSet, sync::LazyLock};
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum DatePatternType {
    NumericSlash,
    NumericSpace,
    NumericConcatenated,
    DateOfBirthWithDashes,
    DateOfBirthConcatenated,
    DateOfBirthWithSpaces,
    VerboseDateShortMonth,
    VerboseDateLongMonth,
}
impl fmt::Debug for DatePatternType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatePatternType::NumericSlash => write!(f, "NumericSlash (\\d{{2}}[-/\\.]\\d{{2}}[-/\\.]\\d{{4}})"),
            DatePatternType::NumericSpace => write!(f, "NumericSpace (\\d{{2}}[-/\\. ]\\d{{2}}[-/\\. ]\\d{{4}})"),
            DatePatternType::NumericConcatenated => write!(f, "NumericConcatenated (\\d{{8}})"),
            DatePatternType::DateOfBirthWithDashes => write!(f, "DateOfBirthWithDashes (Date of Birth: ?(\\d{{2}}[./-]\\d{{2}}[./-]\\d{{4}}))"),
            DatePatternType::DateOfBirthConcatenated => write!(f, "DateOfBirthConcatenated (Date of Birth: ?(\\d{{8}}))"),
            DatePatternType::DateOfBirthWithSpaces => write!(f, "DateOfBirthWithSpaces (Date of Birth: ?(\\d{{2}}[-/\\. ]\\d{{2}}[-/\\. ]\\d{{4}}))"),
            DatePatternType::VerboseDateShortMonth => write!(f, "VerboseDateShortMonth (\\d{{1,2}}\\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\\s+\\d{{4}})"),
            DatePatternType::VerboseDateLongMonth => write!(f, "VerboseDateLongMonth (\\d{{1,2}}\\s+(January|February|March|April|May|June|July|August|September|October|November|December)\\s+\\d{{4}})"),
        }
    }
}

/// Struct to hold a regex pattern and its type
struct DatePattern {
    regex: Regex,
    pattern_type: DatePatternType,
}

static DATE_PATTERNS: LazyLock<Vec<DatePattern>> = LazyLock::new(|| {
    vec![
    DatePattern {
        regex: Regex::new(r"\b(\d{2}[-/\.]\d{2}[-/\.]\d{4})\b").unwrap(),
        pattern_type: DatePatternType::NumericSlash,
    },
    DatePattern {
        regex: Regex::new(r"\b(\d{2}[-/\. ]\d{2}[-/\. ]\d{4})\b").unwrap(),
        pattern_type: DatePatternType::NumericSpace,
    },
    DatePattern {
        regex: Regex::new(r"\b(\d{8})\b").unwrap(),
        pattern_type: DatePatternType::NumericConcatenated,
    },
    DatePattern {
        regex: Regex::new(r"Date of Birth: ?(\d{2}[./-]\d{2}[./-]\d{4})").unwrap(),
        pattern_type: DatePatternType::DateOfBirthWithDashes,
    },
    DatePattern {
        regex: Regex::new(r"Date of Birth: ?(\d{8})").unwrap(),
        pattern_type: DatePatternType::DateOfBirthConcatenated,
    },
    DatePattern {
        regex: Regex::new(r"Date of Birth: ?(\d{2}[-/\. ]\d{2}[-/\. ]\d{4})").unwrap(),
        pattern_type: DatePatternType::DateOfBirthWithSpaces,
    },
    DatePattern {
        regex: Regex::new(r"\b(\d{1,2})\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\s+(\d{4})\b").unwrap(),
        pattern_type: DatePatternType::VerboseDateShortMonth,
    },
    DatePattern {
        regex: Regex::new(r"\b(\d{1,2})\s+(January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{4})\b").unwrap(),
        pattern_type: DatePatternType::VerboseDateLongMonth,
    }
]
});
pub fn find_dates(text: String) -> (HashSet<(String, DatePatternType)>, bool) {
    let mut dates = HashSet::new();
    for pattern in DATE_PATTERNS.iter() {
        for cap in pattern.regex.captures_iter(text.as_str()) {
            dates.insert((cap[0].to_string(), pattern.pattern_type));
        }
    }
    let found_any = !dates.is_empty();
    (dates, found_any)
}
trait DateInfo {
    fn description(&self) -> String;
}

impl DateInfo for DatePatternType {
    fn description(&self) -> String {
        match self {
            DatePatternType::NumericSlash => {
                "Dates with slashes or dots between numbers".to_string()
            }
            DatePatternType::NumericSpace => "Dates with spaces between numbers".to_string(),
            DatePatternType::NumericConcatenated => {
                "Concatenated numeric dates without separators".to_string()
            }
            DatePatternType::DateOfBirthWithDashes => {
                "Date of birth with dashes or dots as separators".to_string()
            }
            DatePatternType::DateOfBirthConcatenated => {
                "Concatenated date of birth without separators".to_string()
            }
            DatePatternType::DateOfBirthWithSpaces => {
                "Date of birth with spaces as separators".to_string()
            }
            DatePatternType::VerboseDateShortMonth => {
                "Verbose dates with short month names".to_string()
            }
            DatePatternType::VerboseDateLongMonth => {
                "Verbose dates with full month names".to_string()
            }
        }
    }
}
