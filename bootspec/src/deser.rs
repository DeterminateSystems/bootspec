use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{Deserializer, MapAccess, Visitor};

use crate::Extension;

struct BootSpecExtensionMapVisitor {
    marker: PhantomData<fn() -> HashMap<String, Extension>>,
}

impl BootSpecExtensionMapVisitor {
    fn new() -> Self {
        BootSpecExtensionMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for BootSpecExtensionMapVisitor {
    type Value = HashMap<String, Extension>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map of bootspec extensions")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry::<String, Extension>()? {
            // FIXME: https://github.com/serde-rs/serde/issues/2200
            // FIXME: implement a deserializer for BootJson and keep track of the parsed field names
            if ["org.nixos.bootspec.v1", "org.nixos.specialisation.v1"].contains(&key.as_str()) {
                continue;
            }

            map.insert(key, value);
        }

        Ok(map)
    }
}

pub fn temp_serde_fix<'de, D>(deserializer: D) -> Result<HashMap<String, Extension>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(BootSpecExtensionMapVisitor::new())
}
