# synthesize

The `synthesize` tool is used to generate a [bootspec] document that can be
consumed in any tool that conforms to the specification. This tool is mostly
useful for creating a bootspec on generations realised prior to the
implementation of the bootspec in NixOS.

See: https://github.com/NixOS/rfcs/pull/125

## Usage

```terminal
$ synthesize /path/to/generation boot.json --version $bootspec_version
```

where `$bootspec_version` is a number referring to the bootspec version you want to synthesize.
