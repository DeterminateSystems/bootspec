use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::generation::Generation;

mod deser;
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
/// The type for generic extensions
pub type Extension = HashMap<String, serde_json::Value>;

// !!! IMPORTANT: KEEP `BootSpec`, `Specialisations`, and `SCHEMA_VERSION` IN SYNC !!!
// The current bootspec generation type
pub type BootSpec = v1::GenerationV1;
/// The current specialisations type
pub type Specialisations = v1::SpecialisationsV1;
/// The current bootspec schema version.
pub const SCHEMA_VERSION: u64 = v1::SCHEMA_VERSION;

/// The current bootspec schema.
// FIXME: manually implement ser/de because otherwise, everything from the generation will exist as an extension too
// FIXME: https://github.com/serde-rs/serde/issues/2200
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootJson {
    #[serde(flatten)]
    pub generation: Generation,
    #[serde(
        default = "HashMap::new",
        skip_serializing_if = "HashMap::is_empty",
        deserialize_with = "deser::temp_serde_fix",
        flatten
    )]
    pub extensions: HashMap<String, Extension>,
}

impl BootJson {
    /// Synthesize a [`BootJson`] struct from the path to a generation.
    ///
    /// This is useful when used on generations that do not have a bootspec attached to it.
    /// This will not synthesize arbitrary extensions.
    pub fn synthesize(generation_path: &Path, version: u64) -> Result<BootJson> {
        let generation = match version {
            v1::SCHEMA_VERSION => {
                let generation = v1::GenerationV1::synthesize(generation_path)?;
                Generation::V1(generation)
            }
            v => {
                return Err(
                    format!("Cannot synthesize for unsupported schema version {}", v).into(),
                )
            }
        };

        Ok(BootJson {
            generation,
            // Extensions will not be synthesized.
            extensions: HashMap::new(),
        })
    }
}
