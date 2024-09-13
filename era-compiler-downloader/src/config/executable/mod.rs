//!
//! The compiler downloader executable config.
//!

pub mod protocol;

use serde::Deserialize;

use self::protocol::Protocol;

///
/// The compiler downloader executable config.
///
#[derive(Debug, Deserialize)]
pub struct Executable {
    /// Whether downloading the executable is enabled.
    pub is_enabled: bool,
    /// The downloading protocol.
    pub protocol: Protocol,
    /// The downloaded data source.
    pub source: String,
    /// The downloaded executable file destination.
    pub destination: String,
}
