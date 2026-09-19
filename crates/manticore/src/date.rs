use serde::Deserialize;
use std::num::ParseIntError;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Clone, Copy)]
#[serde(try_from = "String")]
pub struct YearMonth {
    year: u16,
    month: u8,
}

#[derive(Debug, PartialEq, Error)]
pub enum YearMonthError {
    #[error("invalid format {0}, expected YYYY-MM")]
    BadFormat(String),
    #[error("invalid month {0}, expected 1 - 12")]
    InvalidMonthError(u8),
    #[error("failed to parse number: {0}")]
    ParseInt(#[from] ParseIntError),
}

impl FromStr for YearMonth {
    type Err = YearMonthError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((y, m)) = s.split_once('-') {
            let y: u16 = y.parse()?;
            let m: u8 = m.parse()?;
            if m > 12 {
                Err(Self::Err::InvalidMonthError(m))
            } else {
                Ok(YearMonth { year: y, month: m })
            }
        } else {
            Err(Self::Err::BadFormat(s.to_string()))
        }
    }
}

impl TryFrom<String> for YearMonth {
    type Error = YearMonthError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}-{}", self.year, self.month)
    }
}

#[cfg(test)]
mod tests {
    use std::matches;
    // use thiserror::Error;
    use super::*;

    #[test]
    fn parses_valid_year_month() {
        let ym: YearMonth = "2025-08".parse().unwrap();
        assert_eq!(ym.year, 2025);
        assert_eq!(ym.month, 8);
    }

    #[test]
    fn failed_parses() {
        let bf = "212a";
        assert!(matches!(
            bf.parse::<YearMonth>(),
            Err(YearMonthError::BadFormat(_))
        ))
    }
}
