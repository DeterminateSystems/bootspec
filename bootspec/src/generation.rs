use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::v1;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
/// An enum of all available bootspec versions.
///
/// This enum should be used when attempting to serialize or deserialize a bootspec document, in
/// order to verify the contents match the version of the document.
///
/// This enum is nonexhaustive, because there may be future versions added at any point, and tools
/// should explicitly handle them (e.g. by noting they're currently unsupported).
pub enum Generation<Extension: Default = HashMap<String, serde_json::Value>> {
    V1(v1::GenerationV1<Extension>),
}

impl<Extension: Default> Generation<Extension> {
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

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
    struct TestExtension {
        #[serde(rename = "org.test")]
        test: String,
    }

    #[test]
    fn valid_v1_json() {
        let json = r#"{
    "v1": {
        "system": "x86_64-linux",
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
        "specialisation": {},
        "extensions": {}
    }
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
            extensions: HashMap::new(),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn valid_v1_json_with_typed_extension() {
        let json = r#"{
    "v1": {
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
        "specialisation": {},
        "extensions": { "org.test": "hello" }
    }
}"#;

        let from_json: Generation<TestExtension> = serde_json::from_str(&json).unwrap();
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
            extensions: TestExtension {
                test: "hello".to_string(),
            },
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn valid_v1_json_without_extension() {
        let json = r#"{
    "v1": {
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
    }
}"#;

        let from_json: Generation = serde_json::from_str(&json).unwrap();
        let Generation::V1(from_json) = from_json;

        let expected = crate::v1::GenerationV1 {
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
            specialisation: HashMap::new(),
            extensions: HashMap::new(),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn valid_v1_json_without_initrd() {
        let json = r#"{
    "v1": {
        "system": "x86_64-linux",
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
        "toplevel": "/nix/store/xxx-nixos-system-xxx",
        "specialisation": {}
    }
}"#;

        let from_json: Generation = serde_json::from_str(&json).unwrap();
        let Generation::V1(from_json) = from_json;

        let expected = crate::v1::GenerationV1 {
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
            specialisation: HashMap::new(),
            toplevel: SystemConfigurationRoot(PathBuf::from("/nix/store/xxx-nixos-system-xxx")),
        };

        assert_eq!(from_json, expected);
    }

    #[test]
    fn invalid_json_invalid_version() {
        let json = format!(
            r#"{{
    "v{}": {{
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
    }}
}}"#,
            SCHEMA_VERSION + 1
        );

        let json_err = serde_json::from_str::<Generation>(&json).unwrap_err();
        assert!(json_err.to_string().contains("unknown variant"));
    }
}
