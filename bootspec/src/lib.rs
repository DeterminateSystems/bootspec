use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod generation;
pub mod v1;

#[doc(hidden)]
pub type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
/// A wrapper type describing the name of a NixOS specialisation.
pub struct SpecialisationName(pub String);

impl fmt::Display for SpecialisationName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
/// A wrapper type describing the root directory of a NixOS system configuration.
pub struct SystemConfigurationRoot(pub PathBuf);

/// The bootspec schema filename.
pub const JSON_FILENAME: &str = "boot.json";

// !!! IMPORTANT: KEEP `BootJson`, `Extension`, and `SCHEMA_VERSION` IN SYNC !!!
/// The current bootspec schema.
pub type BootJson = v1::GenerationV1;
/// The current Extension type.
pub type Extension = v1::Extension;
/// The current bootspec schema version.
pub const SCHEMA_VERSION: u64 = v1::SCHEMA_VERSION;

// Enable conversions from Generation into the current Bootspec schema.
impl TryFrom<generation::Generation> for BootJson {
    type Error = &'static str;

    fn try_from(value: generation::Generation) -> Result<Self, Self::Error> {
        match value {
            generation::Generation::V1(boot_json) => Ok(boot_json),
        }
    }
}
