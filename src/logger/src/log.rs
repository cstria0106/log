use std::fmt::Display;

use chrono::{DateTime, Utc};
use colored::*;
use toml::{de::Error, Value};
use toml_highlighter::Highlighter;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl LogLevel {
    fn color(&self) -> &str {
        match self {
            LogLevel::Info => "cyan",
            LogLevel::Warning => "yellow",
            LogLevel::Error => "red",
            LogLevel::Debug => "green",
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Log {
    level: LogLevel,
    message: String,
    timestamp: DateTime<Utc>,
    other: Option<Vec<toml::Value>>,
}

impl Log {
    pub fn new(
        level: LogLevel,
        message: &String,
        other: Option<Vec<String>>,
    ) -> (Log, Vec<toml::de::Error>) {
        let other = other.and_then(|other| {
            Some(
                other
                    .iter()
                    .map(|other| toml::from_str(other))
                    .collect::<Vec<_>>(),
            )
        });

        let (other_values, errors) = if let Some(other) = other {
            let mut values: Vec<Value> = Vec::new();
            let mut errors: Vec<Error> = Vec::new();

            for other in other.into_iter() {
                match other {
                    Ok(value) => {
                        values.push(value);
                    }
                    Err(e) => {
                        errors.push(e);
                    }
                }
            }

            (Some(values), Some(errors))
        } else {
            (None, None)
        };

        (
            Log {
                level,
                message: message.clone(),
                timestamp: Utc::now(),
                other: other_values,
            },
            if let Some(errors) = errors {
                errors
            } else {
                Vec::new()
            },
        )
    }

    pub fn level(&self) -> &LogLevel {
        &self.level
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn to_pretty_string(&self, highlighter: &Highlighter) -> String {
        let message: String = self.message.split('\n').map(|line| line.trim()).collect();
        let other: Option<Vec<String>> = {
            if let Some(other) = &self.other {
                Some(
                    other
                        .iter()
                        .map(|other| -> String {
                            let space = " ".repeat(10);

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
            "{:>9} {} {}{}",
            format!("[{}]", self.level.to_string().to_uppercase()).color(self.level.color()),
            message,
            self.timestamp
                .format("%F %T")
                .to_string()
                .bright_black()
                .to_string(),
            if let Some(other) = other {
                format!("\n{}", other.join("\n"))
            } else {
                "".to_string()
            },
        )
    }
}
