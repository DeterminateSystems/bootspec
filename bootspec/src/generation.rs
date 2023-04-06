use serde::{Deserialize, Serialize};

use crate::v1;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]
#[serde(untagged)]
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::Generation;
    use crate::SystemConfigurationRoot;
    use crate::SCHEMA_VERSION;

    use serde::de::IntoDeserializer;
    use serde::{Deserialize, Serialize};

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

        let bootspec = crate::v1::BootSpecV1 {
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
        let expected = crate::v1::GenerationV1 {
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

        let from_json: crate::BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = crate::v1::BootSpecV1 {
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
        let generation = crate::v1::GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = crate::BootJson {
            generation: crate::Generation::V1(generation),
            extensions: HashMap::from([(
                "org.test".into(),
                HashMap::from([("key".into(), serde_json::to_value("hello").unwrap())]),
            )]),
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
    "org.nixos.specialisation.v1": {},
}"#;

        let from_json: crate::BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = crate::v1::BootSpecV1 {
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
        let generation = crate::v1::GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = crate::BootJson {
            generation: crate::Generation::V1(generation),
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
    "org.test": null
}"#;
        let json_err = serde_json::from_str::<crate::BootJson>(&json).unwrap_err();
        assert!(json_err.to_string().contains("expected a map"));
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
        "toplevel": "/nix/store/xxx-nixos-system-xxx",
    },
    "org.nixos.specialisation.v1": {},
}"#;

        let from_json: crate::BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = crate::v1::BootSpecV1 {
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
        let generation = crate::v1::GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = crate::BootJson {
            generation: crate::Generation::V1(generation),
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

        let from_json: crate::BootJson = serde_json::from_str(&json).unwrap();

        let bootspec = crate::v1::BootSpecV1 {
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
        let generation = crate::v1::GenerationV1 {
            bootspec,
            specialisations: HashMap::new(),
        };
        let expected = crate::BootJson {
            generation: crate::Generation::V1(generation),
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

        let json_err = serde_json::from_str::<crate::v1::GenerationV1>(&json).unwrap_err();
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
}
