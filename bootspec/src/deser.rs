use std::collections::HashMap;
use std::fmt;

use serde::de::{Deserializer, MapAccess, Visitor};

use crate::Extensions;

struct BootSpecExtensionsVisitor;

impl<'de> Visitor<'de> for BootSpecExtensionsVisitor {
    type Value = Extensions;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of bootspec extensions")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry::<String, serde_json::Value>()? {
            // This is very hacky, but necessary because serde does not consume fields in flattened
            // enums (which `Generation` is). Without this, the bootspec and specialisation objects
            // would be duplicated under the `extensions` field.
            // See: https://github.com/serde-rs/serde/issues/2200
            if ["org.nixos.bootspec.", "org.nixos.specialisation."]
                .iter()
                .any(|field| key.starts_with(field))
            {
                continue;
            }

            map.insert(key, value);
        }

        for (k, v) in map.iter() {
            if v.is_null() {
                return Err(serde::de::Error::custom(format!(
                    "{k} was null, but null extensions are not allowed"
                )));
            }
        }

        Ok(map)
    }
}

pub fn skip_generation_fields<'de, D>(deserializer: D) -> Result<Extensions, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(BootSpecExtensionsVisitor)
}
