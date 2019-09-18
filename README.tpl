# {{crate}}

[![Version Badge][badge-version]][bsa-repo]
[![License Badge][badge-license]][bsa-license-file]
[![Github Actions Badge][badge-actions]][bsa-actions]
{{badges}}

[badge-actions]: https://img.shields.io/endpoint.svg?url=https://actions-badge.atrox.dev/CJKay/bsa/badge&label=build
[badge-license]: https://img.shields.io/github/license/CJKay/bsa
[badge-version]: https://img.shields.io/badge/version-{{version}}-grey

[bsa-actions]: https://actions-badge.atrox.dev/CJKay/bsa/goto
[bsa-license-file]: LICENSE.md
[bsa-repo]: https://github.com/CJKay/bsa

{{readme}}

## Safety Dance

[![Unsafe Forbidden Badge][badge-unsafe-forbidden]][safety-dance]

This project subscribes to the principles of the [Rust Safety Dance][safety-dance]. As such, unsafe
code is not permitted and we try to avoid unsafe dependencies where possible. When in doubt,
[`cargo-geiger`][cargo-geiger] to find out!

[badge-unsafe-forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg

[cargo-geiger]: https://github.com/anderejd/cargo-geiger
[safety-dance]: https://github.com/rust-secure-code/safety-dance
