use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

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

// !!! IMPORTANT: KEEP `BootJson`, and `SCHEMA_VERSION` IN SYNC !!!
/// The current Extension type.
pub type Extension = HashMap<String, serde_json::Value>;
pub type BootDocument = v1::GenerationV1;
/// The current bootspec schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootJson {
    #[serde(rename = "org.nixos.bootspec.v1")]
    pub document: BootDocument,
    #[serde(default = "HashMap::new", rename = "org.nixos.specialisation.v1")]
    pub specialisations: HashMap<SpecialisationName, BootDocument>,
    #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty", flatten)]
    pub extensions: HashMap<String, Extension>

}
/// The current bootspec schema version.
pub const SCHEMA_VERSION: u64 = v1::SCHEMA_VERSION;

impl BootJson {
    /// Synthesize a [`BootJson`] struct from the path to a generation.
    ///
    /// This is useful when used on generations that do not have a bootspec attached to it.
    /// This cannot synthesize arbitrary extensions, provide your own custom logic to extend
    /// `extensions` based on existing data.
    fn synthesize(generation: &Path) -> Result<BootJson> {
        let document = BootDocument::synthesize(generation)?;

        let mut specialisations = HashMap::new();
        // Extensions cannot be synthesized.
        let mut extensions = HashMap::new();
        if let Ok(specialisations_dirs) = fs::read_dir(generation.join("specialisation")) {
            for spec in specialisations_dirs.map(|res| res.map(|e| e.path())) {
                let spec = spec?;
                let name = spec
                    .file_name()
                    .ok_or("Could not get name of specialisation dir")?
                    .to_str()
                    .ok_or("Specialisation dir name was invalid UTF8")?;
                let toplevel = fs::canonicalize(generation.join("specialisation").join(name))?;

                specialisations.insert(
                    SpecialisationName(name.to_string()),
                    BootDocument::synthesize(&toplevel)?,
                );
            }
        }

        Ok(BootJson {
            document,
            specialisations,
            extensions
        })
    }
}

// Enable conversions from Generation into the current Bootspec schema.
impl TryFrom<generation::Generation> for BootJson {
    type Error = &'static str;

    fn try_from(value: generation::Generation) -> Result<Self, Self::Error> {
        match value {
            generation::Generation::V1(boot_json) => Ok(boot_json),
        }
    }
}
