use nomad_logger::log;
use nomad_logger::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Foobar {}

#[test]
fn compiles_info() {
    NomadLogger::install_default();
    info!("Fooobar");
    info!("Fooobar {}", "foo");
    info!(Foobar {});
}

#[test]
fn compiles_warn() {
    NomadLogger::install_default();
    warn!("Fooobar");
    warn!("Fooobar {}", "foo");
    warn!(Foobar {});
}

#[test]
fn compiles_trace() {
    NomadLogger::install_default();
    trace!("Fooobar");
    trace!("Fooobar {}", "foo");
    trace!(Foobar {});
}

#[test]
fn compiles_error() {
    NomadLogger::install_default();
    error!("Fooobar");
    error!("Fooobar {}", "foo");
    error!(Foobar {});
}

#[test]
fn compiles_debug() {
    NomadLogger::install_default();
    debug!("Fooobar");
    debug!("Fooobar {}", "foo");
    debug!(Foobar {});
}
