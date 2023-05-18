//! Provides a helper enum for deserializing from all available bootspec versions.
use serde::{Deserialize, Serialize};

use crate::v1;

/// An enum of all available bootspec versions.
///
/// This enum is nonexhaustive, because there may be future versions added at any point, and tools
/// should explicitly handle them (e.g. by noting they're currently unsupported).
///
/// ## Warnings
///
/// If you attempt to deserialize using this struct, you will not get any information about
/// user-provided extensions. For that, you must deserialize with [`crate::BootJson`].
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[serde(untagged)]
pub enum Generation {
    // WARNING: Add new versions to the _top_ of this list. Untagged enums in `serde` always
    // deserialize to the first variant that succeeds, and new versions should succeed before old
    // versions.
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

impl TryFrom<Generation> for v1::GenerationV1 {
    type Error = crate::BootspecError;

    fn try_from(value: Generation) -> Result<Self, Self::Error> {
        let ret = match value {
            Generation::V1(v1) => v1,
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use serde::de::IntoDeserializer;
    use serde::{Deserialize, Serialize};

    use super::Generation;
    use crate::{
        v1::{BootSpecV1, GenerationV1},
        BootJson, SpecialisationName, SystemConfigurationRoot, SCHEMA_VERSION,
    };

    #[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
    struct TestExtension {
        #[serde(rename = "key")]
        test: String,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
    struct TestOptionalExtension {
        #[serde(rename = "key")]
        test: Option<String>,
    }

    #[test]
    fn valid_v1_rfc0125_json() {
        // Adapted from the official JSON5 document from the RFC (converted to JSON and modified to
        // have a valid `org.nixos.specialisation.v1`).
        // https://github.com/NixOS/rfcs/blob/02458c2ecc9f915b143b1923213b40be8ac02a96/rfcs/0125-bootspec.md#bootspec-format-v1
        let rfc_json = include_str!("../rfc0125_spec.json");
        let from_json = serde_json::from_str::<Generation>(&rfc_json).unwrap();
        assert_eq!(from_json.version(), 1);

        let Generation::V1(from_json) = from_json;
        let keys = from_json
            .specialisations
            .keys()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        assert!(keys.contains(&SpecialisationName(String::from("<name>"))));
    }

    #[test]
    fn valid_v1_json_basic() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": {}
}"#;

        let from_json: Generation = serde_json::from_str(&json).unwrap();
        let Generation::V1(from_json) = from_json;

        let bootspec = BootSpecV1 {
            system: String::from("x86_64-linux"),
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
            initrd: Some(PathBuf::from("/nix/store/xxx-initrd-linux/initrd")),
            initrd_secrets: Some(PathBuf::from(
                "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
            )),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };
        let expected = GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn valid_v1_json_with_typed_extension() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": {},
    "org.test": { "key": "hello" }
}"#;

        let from_json: BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = BootSpecV1 {
            system: String::from("x86_64-linux"),
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
            initrd: Some(PathBuf::from("/nix/store/xxx-initrd-linux/initrd")),
            initrd_secrets: Some(PathBuf::from(
                "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
            )),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };
        let generation = GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = BootJson {
            generation: Generation::V1(generation),
            extensions: HashMap::from([("org.test".into(), serde_json::json!({ "key": "hello" }))]),
        };

        let from_extension: TestExtension = Deserialize::deserialize(
            from_json
                .extensions
                .get("org.test")
                .unwrap()
                .to_owned()
                .into_deserializer(),
        )
        .unwrap();
        let expected_extension = TestExtension {
            test: "hello".into(),
        };

        assert_eq!(from_json, expected);
        assert_eq!(from_extension, expected_extension);
    }

    #[test]
    fn valid_v1_json_with_typed_optional_extension_fields_and_empty_object() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": {}
}"#;

        let from_json: BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = BootSpecV1 {
            system: String::from("x86_64-linux"),
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
            initrd: Some(PathBuf::from("/nix/store/xxx-initrd-linux/initrd")),
            initrd_secrets: Some(PathBuf::from(
                "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
            )),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };
        let generation = GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = BootJson {
            generation: Generation::V1(generation),
            extensions: HashMap::new(),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn invalid_v1_json_with_null_extension() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": {},
    "org.test2": { "hi": null },
    "org.test": null
}"#;
        let json_err = serde_json::from_str::<BootJson>(&json).unwrap_err();
        assert!(json_err
            .to_string()
            .contains("org.test was null, but null extensions are not allowed"));
    }

    #[test]
    fn valid_v1_json_without_extension() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": {}
}"#;

        let from_json: BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = BootSpecV1 {
            system: String::from("x86_64-linux"),
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
            initrd: Some(PathBuf::from("/nix/store/xxx-initrd-linux/initrd")),
            initrd_secrets: Some(PathBuf::from(
                "/nix/store/xxx-append-secrets/bin/append-initrd-secrets",
            )),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };
        let generation = GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = BootJson {
            generation: Generation::V1(generation),
            extensions: HashMap::new(),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn valid_v1_json_without_initrd_and_specialisation() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
        "init": "/nix/store/xxx-nixos-system-xxx/init",
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    }
}"#;

        let from_json: BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = BootSpecV1 {
            system: String::from("x86_64-linux"),
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
            initrd: None,
            initrd_secrets: None,
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };
        let generation = GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = BootJson {
            generation: Generation::V1(generation),
            extensions: HashMap::new(),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn invalid_v1_json_with_null_specialisation() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": null
}"#;

        let json_err = serde_json::from_str::<GenerationV1>(&json).unwrap_err();
        assert!(json_err.to_string().contains("expected a map"));
    }

    #[test]
    fn invalid_json_invalid_version() {
        let json = format!(
            r#"{{
    "org.nixos.bootspec.v{}": {{
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    }},
    "org.nixos.specialisation.v{}": {{}}
}}"#,
            SCHEMA_VERSION + 1,
            SCHEMA_VERSION + 1
        );

        let json_err = serde_json::from_str::<Generation>(&json).unwrap_err();
        assert!(json_err.to_string().contains("did not match any variant"));
    }

    #[test]
    fn valid_v1_json_to_generation_via_try_into() {
        let json = r#"{
    "org.nixos.bootspec.v1": {
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
        "system": "x86_64-linux",
        "toplevel": "/nix/store/xxx-nixos-system-xxx"
    },
    "org.nixos.specialisation.v1": {}
}"#;

        let from_json: BootJson = serde_json::from_str(&json).unwrap();
        let _generation: GenerationV1 = from_json.generation.try_into().unwrap();
    }
}
