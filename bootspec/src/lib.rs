mod deser;
pub mod error;
pub mod generation;
pub mod v1;

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{BootspecError, SynthesizeError};
use crate::generation::Generation;

#[doc(hidden)]
pub(crate) type Result<T, E = BootspecError> = core::result::Result<T, E>;

/// A wrapper type describing the name of a NixOS specialisation.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SpecialisationName(pub String);

impl fmt::Display for SpecialisationName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A wrapper type describing the root directory of a NixOS system configuration.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SystemConfigurationRoot(pub PathBuf);

/// The bootspec schema filename.
pub const JSON_FILENAME: &str = "boot.json";
/// The type for a collection of generic extensions.
pub type Extensions = HashMap<String, serde_json::Value>;

// !!! IMPORTANT: KEEP `BootSpec`, `Specialisations`, and `SCHEMA_VERSION` IN SYNC !!!
/// The current bootspec generation type.
pub type BootSpec = v1::GenerationV1;
/// The current specialisations type.
pub type Specialisations = v1::SpecialisationsV1;
/// The current bootspec schema version.
pub const SCHEMA_VERSION: u64 = v1::SCHEMA_VERSION;

/// The current bootspec schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootJson {
    #[serde(flatten)]
    pub generation: Generation,
    #[serde(
        default = "HashMap::new",
        skip_serializing_if = "HashMap::is_empty",
        deserialize_with = "deser::skip_generation_fields",
        flatten
    )]
    pub extensions: Extensions,
}

impl BootJson {
    /// Synthesize a [`BootJson`] struct from the path to a generation using the latest
    /// specification version defined in this crate ([`SCHEMA_VERSION`]).
    ///
    /// See also [`BootJson::synthesize_version`].
    ///
    /// ## Warnings
    ///
    /// Extensions will not be synthesized and will be an empty [`HashMap`].
    pub fn synthesize_latest(generation_path: &Path) -> Result<BootJson> {
        Self::synthesize_version(generation_path, SCHEMA_VERSION)
    }

    /// Synthesize a [`BootJson`] struct from the path to a generation and a specific version.
    ///
    /// This is useful when used on generations that do not have a bootspec attached to it.
    /// This will not synthesize arbitrary extensions.
    ///
    /// ## Warnings
    ///
    /// Extensions will not be synthesized and will be an empty [`HashMap`].
    pub fn synthesize_version(generation_path: &Path, version: u64) -> Result<BootJson> {
        let generation = match version {
            v1::SCHEMA_VERSION => {
                let generation = v1::GenerationV1::synthesize(generation_path)?;
                Generation::V1(generation)
            }
            v => {
                return Err(BootspecError::Synthesize(
                    SynthesizeError::UnsupportedVersion(v),
                ))
            }
        };

        Ok(BootJson {
            generation,
            extensions: HashMap::new(),
        })
    }
}
