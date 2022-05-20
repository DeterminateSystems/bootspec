# bootspec

This repository implements datatypes for NixOS RFC-0125 "bootspec" and a synthesis tool to generate bootspec documents for generations which don't have one.

## Crates

### `bootspec`

The `bootspec` crate provides various structures and constants useful for interacting with the NixOS boot specification.

### `synthesize`

The `synthesize` crate provides a CLI that, when provided a path to a NixOS generation and an output file, will synthesize a boot specification document from the available information.

Verify changes to the synthesis tool with `cargo test` and also by running `./synthesize/integration-test-cases/verify.sh` to ensure it generates the same results as before.

# License

[MIT](./LICENSE)
