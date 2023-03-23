fern
====
[![crates.io version badge][cratesio-badge]][fern-crate]
[![Build Status][test-status-badge]][test-status-link]
[![Average time to resolve an issue][issue-resolution-badge]][isitmaintained-link]
[![Coverage Status][coveralls-badge]][coveralls-builds]

Simple, efficient logging for [Rust].

---

## fern 0.6.* security warning - `colored` feature + global allocator

One of our downstream dependencies, [atty](https://docs.rs/atty/), through
[colored](https://docs.rs/colored/), has a critical security vulnerability:
<https://rustsec.org/advisories/RUSTSEC-2021-0145.html>.

This shows up in one situation: if you're using `colored` (the crate, or our
feature), and a custom global allocator.

I will be releasing `fern` 0.7.0, removing `colored` as a dependency. This may
add another color crate, or may just document usage of alternatives (such as
[`owo-colors`](https://docs.rs/owo-colors/) +
[`enable-ansi-support`](https://docs.rs/enable-ansi-support/0.2.1/le_ansi_support/)).

In the meantime, if you're using `#[global_allocator]`, I highly recommend
removing the `fern/colored` feature.

Or, for minimal code changes, you can also enable the `colored/no-colors`
feature:

```text
cargo add colored --features no-color
```

With the `no-color` feature, the vulnerable code will still be present, but
unless you use any of the following APIs manually, it will never be called:

- [`colored::control::set_override`](https://docs.rs/colored/latest/colored/control/fn.set_override.html)
- [`colored::control::unset_override`](https://docs.rs/colored/latest/colored/control/fn.unset_override.html)
- [`colored::control::ShouldColorize::from_env`](https://docs.rs/colored/latest/colored/control/struct.ShouldColorize.html#method.from_env)
- [`colored::control::SHOULD_COLORIZE`](https://docs.rs/colored/latest/colored/control/struct.SHOULD_COLORIZE.html)
  (referencing this `lazy_static!` variable will initialize it, running the
  vulnerable code)

See <https://github.com/daboross/fern/issues/113> for further discussion.

---

Logging configuration is recursively branched, like a fern: formatting, filters, and output can be applied recursively to match increasingly specific kinds of logging. Fern provides a builder-based configuration backing for rust's standard [log] crate.

```rust
//! With fern, we can:

// Configure logger at runtime
fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "[{} {} {}] {}",
            humantime::format_rfc3339(std::time::SystemTime::now()),
            record.level(),
            record.target(),
            message
        ))
    })
    // Add blanket level filter -
    .level(log::LevelFilter::Debug)
    // - and per-module overrides
    .level_for("hyper", log::LevelFilter::Info)
    // Output to stdout, files, and other Dispatch configurations
    .chain(std::io::stdout())
    .chain(fern::log_file("output.log")?)
    // Apply globally
    .apply()?;

// and log using log crate macros!
info!("hello, world!");
```

Examples of all features at the [api docs][fern-docs]. See fern in use with this [example command line program][fern-example].

---

- [documentation][fern-docs]
- [crates.io page][fern-crate]
- [example program][fern-example]

### Project Status

The fern project is primarily maintained by myself, @daboross on GitHub. It's a hobby project, but one I aim to keep at a high quality.

### Contributing

As this is a hobby project, contributions are very welcome!

The easiest way for you to contribute right now is to use fern in your application, and see where it's lacking. The current library has a solid base, but it lacks features, and I may not anticipate your use cases.

If you have a use case fern does not cover, please file an issue. This is immensely useful to me, to anyone wanting to contribute to the project, and to you as well if the feature is implemented.

If you're interested in helping fix an [existing issue](https://github.com/daboross/fern/issues), or an issue you just filed, help is appreciated.

See [CONTRIBUTING](./CONTRIBUTING.md) for technical information on contributing.

[Rust]: https://www.rust-lang.org/
[test-status-badge]: https://github.com/daboross/fern/workflows/tests/badge.svg?branch=main&event=push
[test-status-link]: https://github.com/daboross/fern/actions/workflows/rust.yml
[issue-resolution-badge]: http://isitmaintained.com/badge/resolution/daboross/fern.svg
[isitmaintained-link]: http://isitmaintained.com/project/daboross/fern
[coveralls-badge]: https://coveralls.io/repos/github/daboross/fern/badge.svg
[coveralls-builds]: https://coveralls.io/github/daboross/fern
[cratesio-badge]: https://img.shields.io/crates/v/fern.svg
[fern-docs]: https://docs.rs/fern/
[fern-crate]: https://crates.io/crates/fern
[fern-example]: https://github.com/daboross/fern/tree/main/examples/cmd-program.rs
[log]: https://github.com/rust-lang/log
