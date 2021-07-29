use std::fmt::Debug;

use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use serde::Serialize;

#[macro_export]
macro_rules! info {
    ($e: expr, $($arg:tt)+) => {
        nomad_log!(log::Level::Info, $e, $($arg)+);
    };

    ($e: expr) => {
        nomad_log!(log::Level::Info, $e);
    };
}

#[macro_export]
macro_rules! error {
    ($e: expr, $($arg:tt)+) => {
        nomad_log!(log::Level::Error, $e,$($arg)+);
    };

    ($e: expr) => {
        nomad_log!(log::Level::Error, $e);
    };
}

#[macro_export]
macro_rules! warn {
    ($e: expr, $($arg:tt)+) => {
        nomad_log!(log::Level::Warn, $e,$($arg)+);
    };

    ($e: expr) => {
        nomad_log!(log::Level::Warn, $e);
    };
}

#[macro_export]
macro_rules! debug {
    ($e: expr, $($arg:tt)+) => {
        nomad_log!(log::Level::Debug, $e,$($arg)+);
    };
    ($e: expr) => {
        nomad_log!(log::Level::Debug, $e);
    };
}

#[macro_export]
macro_rules! trace {
    ($e: expr, $($arg:tt)+) => {
        nomad_log!(log::Level::Trace, $e,$($arg)+);
    };
    ($e: expr) => {
        nomad_log!(log::Level::Trace, $e);
    };
}

#[macro_export]
macro_rules! nomad_log {
    ($lvl:expr, $e: expr, $($arg:tt)+) => {
        log::log!($lvl, $e, $($arg)+);
    };
    ($lvl:expr, $e: expr) => {
            let as_any = (&$e as &dyn std::any::Any);
            if let Some(_f) = as_any.downcast_ref::<String>() {
                log::log!($lvl, "{}", _f);
            } else if let Some(_f) = as_any.downcast_ref::<&str>() {
                log::log!($lvl, "{}", _f);
            } else {
                 crate::nomad_log_serializable!($lvl, $e, Serialize)
            }
    };
}

#[macro_export]
macro_rules! nomad_log_serializable {
    ($lvl:expr, $e: expr, Serialize) => {
        log::log!($lvl, "{}", serde_json::to_string(&$e).unwrap());
    };
}

#[derive(Debug, Serialize)]
struct NomadLog {
    timestamp: i64,
    log_level: String,
    data: serde_json::Value,
}

pub fn setup() {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    stable_eyre::install().unwrap();
    NomadLogger::default().init().unwrap();
}

#[derive(Debug)]
struct NomadLogger {
    max_log_level: Level,
}

impl Default for NomadLogger {
    fn default() -> Self {
        Self {
            max_log_level: Level::Info,
        }
    }
}

impl log::Log for NomadLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log = self.format_log(record);
            println!("{}", serde_json::to_string(&log).unwrap());
        }
    }

    fn flush(&self) {}
}

impl NomadLogger {
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(LevelFilter::Info))
    }

    fn format_log(&self, record: &Record) -> NomadLog {
        let arg = format!("{}", record.args());
        let json: Result<serde_json::Value, _> = serde_json::from_str(&arg);
        if let Ok(json) = json {
            NomadLog {
                data: json,
                log_level: record.level().to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
            }
        } else {
            NomadLog {
                data: serde_json::to_value(arg).expect("Failed to parse log data"),
                log_level: record.level().to_string(),
                timestamp: chrono::Utc::now().timestamp_millis(),
            }
        }
    }
}
