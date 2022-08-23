pub use log::log;
pub use log::Level;

use lazy_static::lazy_static;
use log::{Metadata, Record, SetLoggerError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[macro_export]
macro_rules! info {
    ($e: expr, $($arg:tt)+) => {
        nomad_logger::nomad_log!(log::Level::Info, $e, $($arg)+);
    };

    ($e: expr) => {
        nomad_logger::nomad_log!(log::Level::Info, $e);
    };
}

#[macro_export]
macro_rules! error {
    ($e: expr, $($arg:tt)+) => {
        nomad_logger::nomad_log!(log::Level::Error, $e,$($arg)+);
    };

    ($e: expr) => {
        nomad_logger::nomad_log!(log::Level::Error, $e);
    };
}

#[macro_export]
macro_rules! warn {
    ($e: expr, $($arg:tt)+) => {
        nomad_logger::nomad_log!(log::Level::Warn, $e,$($arg)+);
    };

    ($e: expr) => {
        nomad_logger::nomad_log!(log::Level::Warn, $e);
    };
}

#[macro_export]
macro_rules! debug {
    ($e: expr, $($arg:tt)+) => {
        nomad_logger::nomad_log!(log::Level::Debug, $e,$($arg)+);
    };
    ($e: expr) => {
        nomad_logger::nomad_log!(log::Level::Debug, $e);
    };
}

#[macro_export]
macro_rules! trace {
    ($e: expr, $($arg:tt)+) => {
        nomad_logger::nomad_log!(log::Level::Trace, $e,$($arg)+);
    };
    ($e: expr) => {
        nomad_logger::nomad_log!(log::Level::Trace, $e);
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
                nomad_logger::log!($lvl, "{}", _f);
            } else if let Some(_f) = as_any.downcast_ref::<&str>() {
                nomad_logger::log!($lvl, "{}", _f);
            } else {
                nomad_logger::nomad_log_serializable!($lvl, $e, Serialize)
            }
    };
}

#[macro_export]
macro_rules! nomad_log_serializable {
    ($lvl:expr, $e: expr, Serialize) => {
        nomad_logger::log!($lvl, "{}", serde_json::to_string(&$e).unwrap());
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NomadLog {
    pub timestamp: i64,
    pub log_level: log::Level,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug)]
pub struct NomadLogger {
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
        metadata.level() <= self.max_log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log = self.format_log(record);

            if record.level() == Level::Error {
                eprintln!("{}", serde_json::to_string(&log).unwrap());
            } else {
                println!("{}", serde_json::to_string(&log).unwrap());
            }
        }
    }

    fn flush(&self) {}
}

impl NomadLogger {
    pub fn install_default() {
        if std::env::var("RUST_LIB_BACKTRACE").is_err() {
            std::env::set_var("RUST_LIB_BACKTRACE", "1")
        }
        stable_eyre::install().unwrap();
        NomadLogger::default().init().unwrap();
    }

    pub fn install(self) {
        if std::env::var("RUST_LIB_BACKTRACE").is_err() {
            std::env::set_var("RUST_LIB_BACKTRACE", "1")
        }
        stable_eyre::install().unwrap();
        self.init().unwrap();
    }

    pub fn with_log_level(self, max_log_level: Level) -> Self {
        Self { max_log_level }
    }

    fn init(self) -> Result<(), SetLoggerError> {
        lazy_static! {
            /// This is an example for using doc comment attributes
            static ref INIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        };

        let is_init = INIT.load(std::sync::atomic::Ordering::SeqCst);

        if is_init {
            panic!("Nomad Logger already initialized");
        };

        INIT.store(true, std::sync::atomic::Ordering::SeqCst);

        let filter = self.max_log_level.to_level_filter();
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(filter))
    }

    fn format_log(&self, record: &Record) -> NomadLog {
        let arg = format!("{}", record.args());
        let json: Result<serde_json::Value, _> = serde_json::from_str(&arg);
        if let Ok(json) = json {
            NomadLog {
                data: json,
                log_level: record.level(),
                timestamp: chrono::Utc::now().timestamp_millis(),
            }
        } else {
            NomadLog {
                data: serde_json::to_value(arg).expect("Failed to parse log data"),
                log_level: record.level(),
                timestamp: chrono::Utc::now().timestamp_millis(),
            }
        }
    }
}
