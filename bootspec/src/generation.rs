use serde::de::Error;
use serde::{Deserialize, Serialize};

use crate::v1;
use crate::Result;

#[derive(Debug)]
#[non_exhaustive]
/// An enum of all available bootspec versions.
///
/// This enum should be used when attempting to serialize or deserialize a bootspec document, in
/// order to verify the contents match the version of the document.
///
/// This enum is nonexhaustive, because there may be future versions added at any point, and tools
/// should explicitly handle them (e.g. by noting they're currently unsupported).
pub enum Generation {
    V1(v1::GenerationV1),
}

impl Serialize for Generation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum Generation_<'a> {
            V1(&'a v1::GenerationV1),
        }

        #[derive(Serialize)]
        struct TypedGeneration<'a> {
            #[serde(rename = "schemaVersion")]
            v: u64,
            #[serde(flatten)]
            msg: Generation_<'a>,
        }

        let msg = match self {
            Generation::V1(gen) => TypedGeneration {
                v: v1::SCHEMA_VERSION,
                msg: Generation_::V1(gen),
            },
        };

        msg.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Generation {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;

        let value = Value::deserialize(d)?;

        let gen = match value.get("schemaVersion").and_then(Value::as_u64) {
            Some(v1::SCHEMA_VERSION) => {
                let v1 = v1::GenerationV1::deserialize(value).map_err(D::Error::custom)?;

                Generation::V1(v1)
            }
            Some(ty) => {
                return Err(D::Error::custom(format!(
                    "unsupported schema version {}",
                    ty
                )))
            }
            None => return Err(D::Error::custom("missing / invalid schema version")),
        };

        Ok(gen)
    }
}
