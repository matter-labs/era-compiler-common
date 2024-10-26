//!
//! The compiler downloader executable download protocol.
//!

///
/// The compiler downloader executable download protocol.
///
#[derive(Debug, serde::Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Protocol {
    /// The local file copy.
    #[serde(rename = "file")]
    File,
    /// Download via HTTPS.
    #[serde(rename = "https")]
    HTTPS,
    /// Use the compiler-bin JSON list.
    #[serde(rename = "compiler-bin-list")]
    CompilerBinList,
}
