# bootspec

This crate provides various structures and constants useful for interacting with the NixOS boot specification.

See: https://github.com/NixOS/rfcs/pull/125.

The `BootJson` struct implements the `serde::Deserialize` and `serde::Serialize` traits, making it easy to work with existing bootspec documents as well as creating new ones.
