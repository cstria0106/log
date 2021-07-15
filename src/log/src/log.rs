use std::fmt::Display;

use chrono::{DateTime, Local, NaiveDateTime, ParseError, TimeZone, Utc};
use colored::*;
use toml_highlighter::Highlighter;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum Level {
    Info,
    Warning,
    Error,
    Debug,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Level {
    fn color(&self) -> &str {
        match self {
            Level::Info => "cyan",
            Level::Warning => "yellow",
            Level::Error => "red",
            Level::Debug => "green",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Log {
    level: Level,
    message: String,
    other: Option<Vec<String>>,
    timestamp: DateTime<Utc>,
}

impl Log {
    pub fn from_grpc_log(log: &crate::grpc::Log) -> Result<Self, ParseError> {
        Ok(Self::new(
            match &log.level() {
                crate::grpc::Level::Info => Level::Info,
                crate::grpc::Level::Warning => Level::Warning,
                crate::grpc::Level::Error => Level::Error,
                crate::grpc::Level::Debug => Level::Debug,
            },
            &log.message,
            if log.other.len() > 0 {
                Some(log.other.clone())
            } else {
                None
            },
            Utc.from_local_datetime(&NaiveDateTime::parse_from_str(&log.timestamp, "%F %T")?)
                .unwrap(),
        ))
    }

    pub fn to_grpc_log(&self) -> crate::grpc::Log {
        crate::grpc::Log {
            level: match self.level {
                Level::Info => 0,
                Level::Warning => 1,
                Level::Error => 2,
                Level::Debug => 3,
            },
            message: self.message.clone(),
            other: self.other.clone().unwrap_or(Vec::new()),
            timestamp: self.timestamp.format("%F %T").to_string(),
        }
    }

    pub fn new(
        level: Level,
        message: &String,
        other: Option<Vec<String>>,
        timestamp: DateTime<Utc>,
    ) -> Log {
        Log {
            level,
            message: message.clone(),
            other,
            timestamp,
        }
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn other(&self) -> &Option<Vec<String>> {
        &self.other
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn to_pretty_string(&self, highlighter: &Highlighter) -> String {
        let message: String = self.message.split('\n').map(|line| line.trim()).collect();
        let space_size = 10;
        let other: Option<Vec<String>> = {
            if let Some(other) = &self.other {
                Some(
                    other
                        .iter()
                        .map(|other| -> String {
                            let space = " ".repeat(space_size);

                            let lines: Vec<String> = other
                                .to_string()
                                .trim()
                                .split('\n')
                                .map(|line| format!("{}{}", space, line))
                                .collect();

                            lines.join("\n")
                        })
                        .collect(),
                )
            } else {
                None
            }
        };

        let other = other.and_then(|other: Vec<String>| -> Option<Vec<String>> {
            Some(
                other
                    .iter()
                    .map(|other| highlighter.highlight(other).to_string())
                    .collect(),
            )
        });

        format!(
            "{:>width$} {} {}{}",
            format!("[{}]", self.level.to_string().to_uppercase()).color(self.level.color()),
            message,
            self.timestamp
                .with_timezone(&Local)
                .format("%F %T")
                .to_string()
                .bright_black()
                .to_string(),
            if let Some(other) = other {
                format!("\n{}", other.join("\n"))
            } else {
                "".to_string()
            },
            width = space_size - 1
        )
    }
}
