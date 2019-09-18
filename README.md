# bsa

[![Version Badge][badge-version]][bsa-repo]
[![License Badge][badge-license]][bsa-license-file]
[![Github Actions Badge][badge-actions]][bsa-actions]
[![Coverage Status](https://codecov.io/gh/CJKay/bsa/branch/master/graph/badge.svg)](https://codecov.io/gh/CJKay/bsa)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/CJKay/bsa.svg)](https://isitmaintained.com/project/CJKay/bsa "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/CJKay/bsa.svg)](https://isitmaintained.com/project/CJKay/bsa "Percentage of issues still open")

[badge-actions]: https://img.shields.io/endpoint.svg?url=https://actions-badge.atrox.dev/CJKay/bsa/badge&label=build
[badge-license]: https://img.shields.io/github/license/CJKay/bsa
[badge-version]: https://img.shields.io/badge/version-0.1.0-grey

[bsa-actions]: https://actions-badge.atrox.dev/CJKay/bsa/goto
[bsa-license-file]: LICENSE.md
[bsa-repo]: https://github.com/CJKay/bsa

A library for manipulating Bethesda Softworks Archive (`.bsa`, `.ba2`) files.

```toml
# Cargo.toml

[dependencies]
bsa = { git = "https://github.com/CJKay/bsa.git" }
```

## Overview

`bsa` is a Rust crate for reading and writing Bethesda Softworks Archive (`.bsa`, `.ba2`) files.
These files are used by several Bethesda-developed games to package in-game assets like sounds,
meshes and textures.

Generally speaking, any Bethesda game using [the Gamebryo engine][gamebryo-wiki] or a derivative
of it, such as the [Creation Engine][creation-engine-wiki], uses this archive file format to
package assets.

[creation-engine-wiki]: https://en.wikipedia.org/wiki/Creation_Engine
[gamebryo-wiki]: https://en.wikipedia.org/wiki/Gamebryo

## Safety Dance

[![Unsafe Forbidden Badge][badge-unsafe-forbidden]][safety-dance]

This project subscribes to the principles of the [Rust Safety Dance][safety-dance]. As such, unsafe
code is not permitted and we try to avoid unsafe dependencies where possible. When in doubt,
[`cargo-geiger`][cargo-geiger] to find out!

[badge-unsafe-forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg

[cargo-geiger]: https://github.com/anderejd/cargo-geiger
[safety-dance]: https://github.com/rust-secure-code/safety-dance
