use core::fmt;
use std::{num::ParseIntError, str::FromStr, time::Duration};

const MINUTES: u64 = 60;
const HOURS: u64 = MINUTES * 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Timeout {
    Seconds(i32),
    Minutes(i32),
    Hours(i32),
}

impl Timeout {
    pub fn duration(self) -> Duration {
        match self {
            Timeout::Seconds(t) => Duration::from_secs(t as u64),
            Timeout::Minutes(t) => Duration::from_secs(t as u64 * MINUTES),
            Timeout::Hours(t) => Duration::from_secs(t as u64 * HOURS),
        }
    }
}

impl fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Timeout::Seconds(t) => write!(f, "{t}"),
            Timeout::Minutes(t) => write!(f, "{t}m"),
            Timeout::Hours(t) => write!(f, "{t}h"),
        }
    }
}

impl FromStr for Timeout {
    type Err = ParseTimeoutError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(ParseTimeoutError::Empty);
        }

        let sigil = s
            .chars()
            .rev()
            .find_map(|u| u.is_ascii_alphabetic().then_some(u.to_ascii_lowercase()));

        match sigil {
            Some(sigil) => {
                let value = omit_sigil(s)
                    .parse()
                    .map_err(|e| ParseTimeoutError::Int(s.into(), e))?;
                match sigil {
                    'h' => Ok(Timeout::Hours(value)),
                    'm' => Ok(Timeout::Minutes(value)),
                    's' => Ok(Timeout::Seconds(value)),
                    _ => Err(ParseTimeoutError::Sigil(s.into())),
                }
            }
            None => Ok(Timeout::Seconds(
                s.parse().map_err(|e| ParseTimeoutError::Int(s.into(), e))?,
            )),
        }
    }
}

fn omit_sigil(s: &str) -> &str {
    s.char_indices().next_back().map(|u| &s[..u.0]).unwrap_or(s)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseTimeoutError {
    Empty,
    Sigil(String),
    Int(String, ParseIntError),
}

impl fmt::Display for ParseTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTimeoutError::Empty => f.write_str("empty string"),
            ParseTimeoutError::Sigil(s) => write!(f, "bad sigil in {s:?}"),
            ParseTimeoutError::Int(s, e) => write!(f, "bad integer in {s:?}: {e}"),
        }
    }
}

impl std::error::Error for ParseTimeoutError {}

#[cfg(test)]
mod tests {
    use crate::timeout::Timeout;

    #[test]
    fn omit_sigil() {
        let expected = "5";
        let actual = super::omit_sigil("5m");
        assert_eq!(actual, expected);
    }

    #[test]
    fn can_parse_values() {
        assert_eq!("5m".parse(), Ok(Timeout::Minutes(5)));
        assert_eq!("5s".parse(), Ok(Timeout::Seconds(5)));
        assert_eq!("5".parse(), Ok(Timeout::Seconds(5)));
        assert_eq!("5h".parse(), Ok(Timeout::Hours(5)));
    }
}
