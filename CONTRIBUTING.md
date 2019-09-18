# Contributing Guidelines

Contributions to this project are welcome and encouraged! These guidelines exist to make everybody's
life simpler, regardless of the extent of your involvement in the project.

## Code of Conduct

This project and its participants are governed by the [code of conduct](CODE_OF_CONDUCT.md). By
participating you are expected to uphold this code, so please ensure you read it before
contributing!

## Pull Requests

This section is dedicated to describing the steps you should undertake before submitting a pull
request, or while actively maintaining one.

### Unit, Integration & Documentation Testing

This project uses the built-in Rust testing framework. If you're unsure about how Rust supports
these tests, [Rust by Example][rust-by-example-testing] provides some excellent examples.

You can run all the project's tests with the following:

```
cargo test
```

Before submitting a pull request, it's best ensure all tests are passing locally. The CI will notify
you of any test failures it identifies.

[rust-by-example-testing]: https://doc.rust-lang.org/rust-by-example/testing.html

### ReadMe Generation (`cargo-readme`)

The [`README.md`](README.md) document is automatically generated from the [`README.tpl`](README.tpl)
template file using [`cargo-readme`][cargo-readme]. If you've made changes to the crate
documentation in [`src/lib.rs`](src/lib.rs), you will need to regenerate [`README.md`](README.md).

To do this, make sure you install `cargo-readme` with:

```
cargo install cargo-readme
```

Once that's installed, regenerate the readme with:

```
cargo readme -o README.md
```

The CI will let you know if you have forgotten this step, so don't panic if you've forgotten!

[cargo-readme]: https://github.com/livioribeiro/cargo-readme

### Coding Style & Linting

#### Formatting (`rustfmt`)

This project uses [`rustfmt`][rustfmt] to enforce a coding style. Incidentally, to keep the style as
in-line as possible with accepted Rust conventions, we make no adjustments to the default style. We
do not generally accept deviations, but if you can make an argument for one then feel free.

If your pull request does not align with the coding style, the CI will reject it. If it does, you
can re-format your codebase with:

```
cargo fmt
```

[rustfmt]: https://github.com/rust-lang/rustfmt

#### Linting (`rust-clippy`)

On top of this, we also employ the [`Clippy`][clippy] linter. This helps us identify common
anti-patterns and apply best-practice rules. As with formatting, the CI will reject your pull
request if `Clippy` flags anything up. You can identify any issues yourself with:

```
cargo clippy
```

[clippy]: https://github.com/rust-lang/clippy

## Continuous Integration

### Testing

We use [GitHub Actions][github-actions] to do automated testing and deployment to
[crates.io][crates-io]. Pull requests and commits to the `master` branch undergo unit, integration
and documentation testing.

[crates-io]: https://crates.io
[github-actions]: https://github.com/features/actions

### Code Coverage

Code coverage analysis, like the tests described above, is run on both the `master` branch and
individual pull requests. These coverage results are uploaded to [codecov.io][codecov-io] and you
can find them [here][codecov-results].

[codecov-io]: https://codecov.io
[codecov-results]: https://codecov.io/gh/CJKay/bsa

### Fuzzing

Full fuzz testing happens nightly, and regression tests are run on all pull requests. If you're
interested in reading more about fuzzing, try the [Rust Fuzz Book][rust-fuzz-book]!

[fuzzit]: https://fuzzit.dev
[rust-fuzz-book]: https://rust-fuzz.github.io/book/introduction.html
