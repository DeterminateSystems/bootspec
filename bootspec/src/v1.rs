use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{Result, SpecialisationName, SystemConfigurationRoot};

/// The V1 bootspec schema version.
pub const SCHEMA_VERSION: u64 = 1;

/// A mapping of V1 bootspec specialisations.
///
/// This structure represents the contents of the `org.nixos.specialisations.v1` key.
pub type SpecialisationsV1 = HashMap<SpecialisationName, BootSpecV1>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// A V1 bootspec generation.
///
/// This structure represents an entire V1 generation (i.e. it includes the `org.nixos.bootspec.v1`
/// and `org.nixos.specialisation.v1` structures).
///
/// ## Warnings
///
/// Do not attempt to deserialize this struct from a bootspec document, as it does not enforce
/// versioning. You want to use the [`crate::generation::Generation`] enum for both
/// serialization and deserialization.
pub struct GenerationV1 {
    #[serde(rename = "org.nixos.bootspec.v1")]
    pub bootspec: BootSpecV1,
    #[serde(rename = "org.nixos.specialisation.v1")]
    pub specialisations: SpecialisationsV1,
}

impl GenerationV1 {
    /// Synthesize a [`GenerationV1`] struct from the path to a generation.
    ///
    /// This is useful when used on generations that do not have a bootspec attached to it.
    pub fn synthesize(generation_path: &Path) -> Result<Self> {
        let bootspec = BootSpecV1::synthesize(generation_path)?;

        let mut specialisations = HashMap::new();
        if let Ok(specialisations_dirs) = fs::read_dir(generation_path.join("specialisation")) {
            for specialisation in specialisations_dirs.map(|res| res.map(|e| e.path())) {
                let specialisation = specialisation?;
                let name = specialisation
                    .file_name()
                    .ok_or("Could not get name of specialisation dir")?
                    .to_str()
                    .ok_or("Specialisation dir name was invalid UTF8")?;
                let toplevel = fs::canonicalize(generation_path.join("specialisation").join(name))?;

                specialisations.insert(
                    SpecialisationName(name.to_string()),
                    BootSpecV1::synthesize(&toplevel)?,
                );
            }
        }

        Ok(Self {
            bootspec,
            specialisations,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// A V1 bootspec toplevel.
///
/// This structure represents the contents of the `org.nixos.bootspec.v1` key.
pub struct BootSpecV1 {
    /// Label for the system closure
    pub label: String,
    /// Path to kernel (bzImage) -- $toplevel/kernel
    pub kernel: PathBuf,
    /// list of kernel parameters
    pub kernel_params: Vec<String>,
    /// Path to the init script
    pub init: PathBuf,
    /// Path to initrd -- $toplevel/initrd
    pub initrd: Option<PathBuf>,
    /// Path to "append-initrd-secrets" script -- $toplevel/append-initrd-secrets
    pub initrd_secrets: Option<PathBuf>,
    /// System double, e.g. x86_64-linux, for the system closure
    pub system: String,
    /// config.system.build.toplevel path
    pub toplevel: SystemConfigurationRoot,
}

impl BootSpecV1 {
    pub(crate) fn synthesize(generation: &Path) -> Result<Self> {
        let generation = generation
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize generation dir:\n{}", e))?;

        let system_version = fs::read_to_string(generation.join("nixos-version"))
            .map_err(|e| format!("Failed to read system version:\n{}", e))?;

        let system = fs::read_to_string(generation.join("system"))
            .map_err(|e| format!("Failed to read system double:\n{}", e))?;

        let kernel = fs::canonicalize(generation.join("kernel-modules/bzImage"))
            .map_err(|e| format!("Failed to canonicalize the kernel:\n{}", e))?;

        let kernel_modules = fs::canonicalize(generation.join("kernel-modules/lib/modules"))
            .map_err(|e| format!("Failed to canonicalize the kernel modules dir:\n{}", e))?;
        let versioned_kernel_modules = fs::read_dir(kernel_modules)
            .map_err(|e| format!("Failed to read kernel modules dir:\n{}", e))?
            .map(|res| res.map(|e| e.path()))
            .next()
            .ok_or("Could not find kernel version dir")??;
        let kernel_version = versioned_kernel_modules
            .file_name()
            .ok_or("Could not get name of kernel version dir")?
            .to_str()
            .ok_or("Kernel version dir name was invalid UTF8")?;

        let kernel_params: Vec<String> = fs::read_to_string(generation.join("kernel-params"))?
            .split(' ')
            .map(str::to_string)
            .collect();

        let init = generation.join("init");

        let initrd_path = generation.join("initrd");
        let initrd = if initrd_path.exists() {
            Some(
                fs::canonicalize(initrd_path)
                    .map_err(|e| format!("Failed to canonicalize the initrd:\n{}", e))?,
            )
        } else {
            None
        };

        let initrd_secrets = if generation.join("append-initrd-secrets").exists() {
            Some(generation.join("append-initrd-secrets"))
        } else {
            None
        };

        Ok(Self {
            label: format!("NixOS {} (Linux {})", system_version, kernel_version),
            kernel,
            kernel_params,
            init,
            initrd,
            initrd_secrets,
            system,
            toplevel: SystemConfigurationRoot(generation),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::{BootSpecV1, SystemConfigurationRoot};
    use crate::JSON_FILENAME;
    use tempfile::TempDir;

    fn create_generation_files_and_dirs(
        generation: &PathBuf,
        kernel_version: &str,
        system: &str,
        system_version: &str,
        kernel_params: &Vec<String>,
    ) {
        fs::create_dir_all(
            generation.join(format!("kernel-modules/lib/modules/{}", kernel_version)),
        )
        .expect("Failed to write to test generation");
        fs::create_dir_all(generation.join("specialisation"))
            .expect("Failed to write to test generation");
        fs::create_dir_all(generation.join("bootspec"))
            .expect("Failed to create the bootspec directory during test scaffolding");

        fs::write(generation.join("nixos-version"), system_version)
            .expect("Failed to write to test generation");
        fs::write(generation.join("system"), system).expect("Failed to write system double");
        fs::write(generation.join("kernel-modules/bzImage"), "")
            .expect("Failed to write to test generation");
        fs::write(generation.join("kernel-params"), kernel_params.join(" "))
            .expect("Failed to write to test generation");
        fs::write(generation.join("init"), "").expect("Failed to write to test generation");
        fs::write(generation.join("initrd"), "").expect("Failed to write to test generation");
        fs::write(generation.join("append-initrd-secrets"), "")
            .expect("Failed to write to test generation");
    }

    fn scaffold(
        system: &str,
        system_version: &str,
        kernel_version: &str,
        kernel_params: &Vec<String>,
        specialisations: Option<Vec<&str>>,
        specialisations_have_boot_spec: bool,
    ) -> PathBuf {
        let temp_dir = TempDir::new().expect("Failed to create tempdir for test generation");
        let generation = temp_dir.into_path();

        create_generation_files_and_dirs(
            &generation,
            kernel_version,
            system,
            system_version,
            kernel_params,
        );

        if let Some(specialisations) = specialisations {
            for spec_name in specialisations {
                let spec_path = generation.join("specialisation").join(spec_name);
                fs::create_dir_all(&spec_path).expect("Failed to write to test generation");

                create_generation_files_and_dirs(
                    &spec_path,
                    kernel_version,
                    system_version,
                    system,
                    kernel_params,
                );

                if specialisations_have_boot_spec {
                    fs::write(spec_path.join(JSON_FILENAME), "")
                        .expect("Failed to write to test generation");
                }
            }
        }

        generation
    }

    #[test]
    fn no_bootspec_no_specialisation() {
        let system = String::from("x86_64-linux");
        let system_version = String::from("test-version-1");
        let kernel_version = String::from("1.1.1-test1");
        let kernel_params = [
            "udev.log_priority=3",
            "systemd.unified_cgroup_hierarchy=1",
            "loglevel=4",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

        let generation = scaffold(
            &system,
            &system_version,
            &kernel_version,
            &kernel_params,
            None,
            false,
        );
        let spec = BootSpecV1::synthesize(&generation).unwrap();

        assert_eq!(
            spec,
            BootSpecV1 {
                system,
                label: "NixOS test-version-1 (Linux 1.1.1-test1)".into(),
                kernel: generation.join("kernel-modules/bzImage"),
                kernel_params,
                init: generation.join("init"),
                initrd: Some(generation.join("initrd")),
                initrd_secrets: Some(generation.join("append-initrd-secrets")),
                toplevel: SystemConfigurationRoot(generation),
            }
        );
    }

    #[test]
    fn no_bootspec_with_specialisation_no_bootspec() {
        let system = String::from("x86_64-linux");
        let system_version = String::from("test-version-2");
        let kernel_version = String::from("1.1.1-test2");
        let kernel_params = [
            "udev.log_priority=3",
            "systemd.unified_cgroup_hierarchy=1",
            "loglevel=4",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        let specialisations = vec!["spec1", "spec2"];

        let generation = scaffold(
            &system,
            &system_version,
            &kernel_version,
            &kernel_params,
            Some(specialisations),
            false,
        );

        BootSpecV1::synthesize(&generation).unwrap();
    }

    #[test]
    fn with_bootspec_no_specialisation() {
        let system = String::from("x86_64-linux");
        let system_version = String::from("test-version-3");
        let kernel_version = String::from("1.1.1-test3");
        let kernel_params = [
            "udev.log_priority=3",
            "systemd.unified_cgroup_hierarchy=1",
            "loglevel=4",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

        let generation = scaffold(
            &system,
            &system_version,
            &kernel_version,
            &kernel_params,
            None,
            false,
        );

        fs::write(generation.join(JSON_FILENAME), "").expect("Failed to write to test generation");

        let spec = BootSpecV1::synthesize(&generation).unwrap();

        assert_eq!(
            spec,
            BootSpecV1 {
                system,
                label: "NixOS test-version-3 (Linux 1.1.1-test3)".into(),
                kernel: generation.join("kernel-modules/bzImage"),
                kernel_params,
                init: generation.join("init"),
                initrd: Some(generation.join("initrd")),
                initrd_secrets: Some(generation.join("append-initrd-secrets")),
                toplevel: SystemConfigurationRoot(generation)
            }
        );
    }

    #[test]
    fn with_bootspec_with_specialisations_with_bootspec() {
        let system = String::from("x86_64-linux");
        let system_version = String::from("test-version-4");
        let kernel_version = String::from("1.1.1-test4");
        let kernel_params = [
            "udev.log_priority=3",
            "systemd.unified_cgroup_hierarchy=1",
            "loglevel=4",
        ]
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
        let specialisations = vec!["spec1", "spec2"];

        let generation = scaffold(
            &system,
            &system_version,
            &kernel_version,
            &kernel_params,
            Some(specialisations),
            true,
        );

        fs::write(generation.join("bootspec").join(JSON_FILENAME), "")
            .expect("Failed to write to test generation");

        BootSpecV1::synthesize(&generation).unwrap();
    }
}
