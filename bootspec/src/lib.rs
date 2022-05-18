use std::error::Error;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod v1;

#[doc(hidden)]
pub type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
/// A wrapper type describing the name of a NixOS specialisation.
pub struct SpecialisationName(pub String);

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
/// A wrapper type describing the root directory of a NixOS system configuration.
pub struct SystemConfigurationRoot(pub PathBuf);

// !!! IMPORTANT: KEEP `BootJson`, `SCHEMA_VERSION`, and `JSON_FILENAME` IN SYNC !!!
/// The current bootspec schema.
pub type BootJson = v1::GenerationV1;
/// The current bootspec schema version.
pub const SCHEMA_VERSION: u32 = v1::SCHEMA_VERSION;
/// The current bootspec schema filename.
pub const JSON_FILENAME: &str = v1::JSON_FILENAME;
