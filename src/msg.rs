use std::{time::SystemTime, cmp::Ordering};

use chrono::{DateTime, Utc};
use contra::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum LogIntensity {
    Trace = -2,
    Debug = -1,
    Info = 0,
    Warn = 1,
    Error = 2,
    Fatal = 3,
}

impl ToString for LogIntensity {
    fn to_string(&self) -> String {
        match self {
            LogIntensity::Trace => "Trace".to_string(),
            LogIntensity::Debug => "Debug".to_string(),
            LogIntensity::Info =>  "Info ".to_string(),
            LogIntensity::Warn =>  "Warn ".to_string(),
            LogIntensity::Error => "Error".to_string(),
            LogIntensity::Fatal => "Fatal".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct LogMessage<'a> {
    pub(crate) time: SystemTime, 
    pub(crate) scope: &'a str, 
    pub(crate) file: &'a str, 
    pub(crate) line: u32, 
    pub(crate) msg: &'a str,
    pub(crate) intensity: LogIntensity,
    pub(crate) color: Color,
}

impl<'a> LogMessage<'a> {
    /// Replaces all % patterns with the appropriate content
    /// %t = UTC timestamp
    /// %c = current thread id
    /// %i = log intensity
    /// %m = log message
    /// %f = file
    /// %l = line
    /// %s = scope
    /// %[ = color start
    /// %] = color stop
    #[inline]
    fn replace(&self, c: char, mut parsed: String) -> String {
        match c {
            '[' => parsed.push_str( &self.color.ansi()),
            ']' => parsed.push_str( &Color::Default.ansi()),
            's' => parsed.push_str(self.scope),
            'f' => parsed.push_str(self.file),
            'l' => parsed.push_str(&self.line.to_string()),
            'm' => parsed.push_str(self.msg),
            'i' => parsed.push_str(&self.intensity.to_string()),
            't' =>  parsed.push_str(&DateTime::<Utc>::from(self.time).to_rfc3339()), 
            'c' =>  parsed.push_str(&format!("{:?}",std::thread::current().id())), 
            _ => (),
        };
        parsed
    }

    pub fn parse(&self, pattern: &str) -> String {
        let mut parsed = String::new();

        let mut escaped = false;
        let mut replace = false;

        for c in pattern.chars() {
            if escaped {
                escaped = false;
                parsed.push(c);
                continue;
            }

            if replace {
                replace = false;
                parsed = self.replace(c, parsed);
                continue;
            }

            if c == '\\'{
                escaped = true;
                continue;
            }

            if c == '%' {
                replace = true;
                continue;
            }

            parsed.push(c);
        }

        parsed
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Color {
    Default,
    Grey,
    Blue,
    White,
    Orange,
    Red,
    DarkRed,
}

impl Color {
    pub fn ansi(&self) -> String {
        match self {
            Color::Default => "\x1b[0m".to_string(),
            Color::Grey => "\x1b[90m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::White => "\x1b[97m".to_string(),
            Color::Orange => "\x1b[33m".to_string(),
            Color::Red => "\x1b[31m".to_string(),
            Color::DarkRed => "\x1b[91m".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use crate::msg::{LogMessage, Color};

    #[test]
    fn log_message_parsing_works() {
        let msg = LogMessage {
            time: DateTime::<Utc>::default().into(),
            scope: "logtra",
            file: "lib.rs",
            line: 12,
            msg: "Hello world!",
            intensity: crate::msg::LogIntensity::Info,
            color: Color::Red,
        };

        let result = msg.parse("[%t][%c][%[%s%]][%f:%l]: %m");
        assert_eq!("[1970-01-01T00:00:00+00:00][ThreadId(2)][\x1b[31mlogtra\x1b[0m][lib.rs:12]: Hello world!", &result);
    }
}