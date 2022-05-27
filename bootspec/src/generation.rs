use serde::de::Error as _;
use serde::ser::Error as _;
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

impl Generation {
    /// The version of the bootspec document.
    pub fn version(&self) -> u64 {
        use Generation::*;

        match self {
            V1(_) => v1::SCHEMA_VERSION,
        }
    }
}

impl Serialize for Generation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use Generation::*;

        #[derive(Serialize)]
        struct TypedGeneration {
            #[serde(rename = "schemaVersion")]
            v: u64,
            #[serde(flatten)]
            msg: serde_json::Value,
        }

        let msg = match self {
            V1(gen) => TypedGeneration {
                v: v1::SCHEMA_VERSION,
                msg: serde_json::to_value(gen).map_err(S::Error::custom)?,
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::Generation;
    use crate::SystemConfigurationRoot;
    use crate::SCHEMA_VERSION;

    #[test]
    fn valid_json() {
        let json = r#"{
    "schemaVersion": 1,
    "init": "/nix/store/xxx-nixos-system-xxx/init",
    "initrd": "/nix/store/xxx-initrd-linux/initrd",
    "initrdSecrets": "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
    "kernel": "/nix/store/xxx-linux/bzImage",
    "kernelParams": [
    "amd_iommu=on",
    "amd_iommu=pt",
    "iommu=pt",
    "kvm.ignore_msrs=1",
    "kvm.report_ignored_msrs=0",
    "udev.log_priority=3",
    "systemd.unified_cgroup_hierarchy=1",
    "loglevel=4"
    ],
    "label": "NixOS 21.11.20210810.dirty (Linux 5.15.30)",
    "toplevel": "/nix/store/xxx-nixos-system-xxx",
    "specialisation": {}
}"#;

        let from_json: Generation = serde_json::from_str(&json).unwrap();
        let Generation::V1(from_json) = from_json;

        let expected = crate::v1::GenerationV1 {
            label: String::from("NixOS 21.11.20210810.dirty (Linux 5.15.30)"),
            kernel: PathBuf::from("/nix/store/xxx-linux/bzImage"),
            kernel_params: vec![
                "amd_iommu=on",
                "amd_iommu=pt",
                "iommu=pt",
                "kvm.ignore_msrs=1",
                "kvm.report_ignored_msrs=0",
                "udev.log_priority=3",
                "systemd.unified_cgroup_hierarchy=1",
                "loglevel=4",
            ]
            .iter()
            .map(ToString::to_string)
            .collect(),
            init: PathBuf::from("/nix/store/xxx-nixos-system-xxx/init"),
            initrd: PathBuf::from("/nix/store/xxx-initrd-linux/initrd"),
            initrd_secrets: Some(PathBuf::from(
                "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
            )),
            specialisation: HashMap::new(),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn invalid_json_missing_version() {
        let json = r#"{
    "init": "/nix/store/xxx-nixos-system-xxx/init",
    "initrd": "/nix/store/xxx-initrd-linux/initrd",
    "initrdSecrets": "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
    "kernel": "/nix/store/xxx-linux/bzImage",
    "kernelParams": [
    "amd_iommu=on",
    "amd_iommu=pt",
    "iommu=pt",
    "kvm.ignore_msrs=1",
    "kvm.report_ignored_msrs=0",
    "udev.log_priority=3",
    "systemd.unified_cgroup_hierarchy=1",
    "loglevel=4"
    ],
    "label": "NixOS 21.11.20210810.dirty (Linux 5.15.30)",
    "toplevel": "/nix/store/xxx-nixos-system-xxx",
    "specialisation": {}
}"#;

        let json_err = serde_json::from_str::<Generation>(&json).unwrap_err();
        assert_eq!(json_err.to_string(), "missing / invalid schema version");
    }

    #[test]
    fn invalid_json_invalid_version() {
        let json = format!(
            r#"{{
    "schemaVersion": {},
    "init": "/nix/store/xxx-nixos-system-xxx/init",
    "initrd": "/nix/store/xxx-initrd-linux/initrd",
    "initrdSecrets": "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
    "kernel": "/nix/store/xxx-linux/bzImage",
    "kernelParams": [
    "amd_iommu=on",
    "amd_iommu=pt",
    "iommu=pt",
    "kvm.ignore_msrs=1",
    "kvm.report_ignored_msrs=0",
    "udev.log_priority=3",
    "systemd.unified_cgroup_hierarchy=1",
    "loglevel=4"
    ],
    "label": "NixOS 21.11.20210810.dirty (Linux 5.15.30)",
    "toplevel": "/nix/store/xxx-nixos-system-xxx",
    "specialisation": {{}}
}}"#,
            SCHEMA_VERSION + 1
        );

        let json_err = serde_json::from_str::<Generation>(&json).unwrap_err();
        assert_eq!(
            json_err.to_string(),
            format!("unsupported schema version {}", SCHEMA_VERSION + 1)
        );
    }
}
